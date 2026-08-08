export interface GetInfoResponse {
    title: string,
    public_key: string
}

export interface File {
    id: string,
    name: string,
    mime_type: string,
    size: number,
    hash: string,
}

export async function getInfo(url: string): Promise<GetInfoResponse> {
    const response = await fetch(`${url}/info`, {
        method: "GET",
        headers: { "Content-Type": "application/json" },
    });

    return response.json();
}

export async function postFiles(url: string, files: FileList): Promise<File[]> {
    const formData = new FormData();

    for (const file of files) {
        formData.append(file.name, file);
    }

    const response = await fetch(`${url}/files`, {
        method: "POST",
        body: formData
    });

    return response.json();
}

export async function getFile(url: string, id: string): Promise<Blob> {
    const response = await fetch(`${url}/files/${id}`, {
        method: "GET"
    });

    return response.blob();
}
export const firstFileToByteArray = async (fileList: FileList | null | undefined) => {
    const arrayBuffer = await fileList?.item(0)?.arrayBuffer();
    return arrayBuffer ? Array.from(new Uint8Array(arrayBuffer)) : null;
}

export const byteArrayToBlob = (byteArray: number[]) => {
    return new Blob([new Uint8Array(byteArray)])
}

import { ElIcon, ElImage, ElTable, ElTableColumn, ElUpload, type UploadProps } from "element-plus";
import { Plus } from "@element-plus/icons-vue";
import { defineComponent, onMounted, ref } from "vue";

export default defineComponent({
    setup() {
        const imageUrl = ref()
        const onSuccess: UploadProps['onSuccess'] = (
            response,
            uploadFile
        ) => {
            console.log(response);
            if (response.length > 0) {
                imageUrl.value = `http://localhost:3000/${response[0]}`
            }
        }

        const data = ref([])
        onMounted(async () => {
            const res = await fetch(`http://localhost:3000/list`)
            if (res.ok) {
                data.value = await res.json()
                console.log(data.value);
            }
        })
        return () =>
            <div style={{ minHeight: '100vh', display: 'flex', justifyContent: 'center', alignItems: 'center', flexDirection: 'column', width: '80vw' }}>
                <ElUpload showFileList={false} onSuccess={onSuccess} action={`http://localhost:3000/upload`}>
                    {imageUrl.value ? <ElImage src={imageUrl.value}></ElImage> : <ElIcon><Plus /></ElIcon>}
                </ElUpload>
                <ElTable data={data.value}>
                    <ElTableColumn label={"地址"} formatter={(row) => `http://localhost:3000/${row}`} minWidth={'512'}></ElTableColumn>
                </ElTable>
            </div >
    }
})
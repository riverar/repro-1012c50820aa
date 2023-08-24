use windows::{
    w,
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CLSCTX_INPROC_SERVER},
        UI::Shell::{
            FileOperation, IFileOperation, IShellItem, SHCreateItemFromParsingName,
            FOFX_REQUIREELEVATION, FOFX_SHOWELEVATIONPROMPT, FOF_NOCONFIRMMKDIR, FOF_NOERRORUI,
            FOF_RENAMEONCOLLISION, FOF_SILENT,
        },
    },
};

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitialize(None)?;

        let fo: IFileOperation = CoCreateInstance(&FileOperation, None, CLSCTX_INPROC_SERVER)?;
        fo.SetOperationFlags(
            FOFX_REQUIREELEVATION
                | FOFX_SHOWELEVATIONPROMPT
                | FOF_SILENT
                | FOF_NOCONFIRMMKDIR
                | FOF_RENAMEONCOLLISION
                | FOF_NOERRORUI,
        )?;

        let src: IShellItem = SHCreateItemFromParsingName(w!(r#"C:\Windows\win.ini"#), None)?;
        let dst: IShellItem = SHCreateItemFromParsingName(w!(r#"C:\Windows"#), None)?;
        fo.CopyItem(&src, &dst, w!("test.txt"), None)?;

        dbg!(fo.PerformOperations());

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        Ok(())
    }
}

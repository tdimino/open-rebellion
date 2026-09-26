
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_0061d640(UINT param_1,int param_2,int param_3)

{
  HANDLE hProcess;
  undefined4 *puVar1;
  undefined4 *puVar2;
  UINT uExitCode;
  
  FUN_0061d700();
  if (DAT_006be7c4 == 1) {
    uExitCode = param_1;
    hProcess = GetCurrentProcess();
    TerminateProcess(hProcess,uExitCode);
  }
  _DAT_006be7c0 = 1;
  DAT_006be7bc = (undefined1)param_3;
  if (param_2 == 0) {
    if ((DAT_006bed1c != (undefined4 *)0x0) &&
       (puVar2 = (undefined4 *)(DAT_006bed18 + -4), puVar1 = DAT_006bed1c, DAT_006bed1c <= puVar2))
    {
      do {
        if ((code *)*puVar2 != (code *)0x0) {
          (*(code *)*puVar2)();
          puVar1 = DAT_006bed1c;
        }
        puVar2 = puVar2 + -1;
      } while (puVar1 <= puVar2);
    }
    FUN_0061d720((undefined4 *)&DAT_006a7bf0,(undefined4 *)&DAT_006a7bf8);
  }
  FUN_0061d720((undefined4 *)&DAT_006a7bfc,(undefined4 *)&DAT_006a7c04);
  if (param_3 != 0) {
    FUN_0061d710();
    return;
  }
  DAT_006be7c4 = 1;
                    /* WARNING: Subroutine does not return */
  ExitProcess(param_1);
}


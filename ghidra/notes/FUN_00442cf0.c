
void __fastcall FUN_00442cf0(int *param_1)

{
  if (param_1 == DAT_006b28f4) {
    PostMessageA((HWND)param_1[6],0x405,(WPARAM)param_1[6],0);
    return;
  }
                    /* WARNING: Could not recover jumptable at 0x00442d0d. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(*param_1 + 8))();
  return;
}


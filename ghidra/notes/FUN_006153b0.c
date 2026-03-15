
undefined4 * __fastcall FUN_006153b0(undefined4 *param_1)

{
  int iVar1;
  bool bVar2;
  
  param_1[1] = 0;
  param_1[3] = 0;
  param_1[4] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[0xc] = 0;
  param_1[7] = 0;
  *param_1 = &PTR_FUN_0066e4dc;
  param_1[2] = 4;
  param_1[10] = 6;
  *(undefined1 *)(param_1 + 0xb) = 0x20;
  param_1[0xd] = 0xffffffff;
  FUN_00615ec0((LPCRITICAL_SECTION)(param_1 + 0xe));
  iVar1 = DAT_006be740 + 1;
  bVar2 = DAT_006be740 == 0;
  DAT_006be740 = iVar1;
  if (bVar2) {
    FUN_00615ec0((LPCRITICAL_SECTION)&DAT_006be708);
  }
  return param_1;
}


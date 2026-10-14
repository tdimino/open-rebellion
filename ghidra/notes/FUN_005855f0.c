
undefined4 * __fastcall FUN_005855f0(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  
  FUN_0058f7b0(param_1);
  *param_1 = &PTR_FUN_0066a1f0;
  puVar2 = param_1 + 9;
  for (iVar1 = 8; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  return param_1;
}


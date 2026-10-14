
bool __fastcall FUN_004fc5c0(int *param_1)

{
  int iVar1;
  
  iVar1 = (**(code **)(*param_1 + 0xe8))();
  param_1[0x15] = iVar1;
  return iVar1 != 0;
}


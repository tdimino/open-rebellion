
undefined4 __cdecl
FUN_0061cf10(DWORD param_1,undefined4 param_2,DWORD param_3,undefined4 param_4,undefined4 param_5,
            int param_6,int param_7)

{
  DWORD *pDVar1;
  undefined4 uVar2;
  void *local_14;
  code *pcStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  local_8 = 0xffffffff;
  puStack_c = &DAT_0066e6d8;
  pcStack_10 = FUN_0061a414;
  local_14 = ExceptionList;
  ExceptionList = &local_14;
  FUN_0061d430();
  FUN_0061d430();
  pDVar1 = FUN_0061d430();
  pDVar1[0x1b] = param_1;
  pDVar1 = FUN_0061d430();
  pDVar1[0x1c] = param_3;
  local_8 = 1;
  uVar2 = FUN_00618800(param_2,param_4,param_5,param_6,param_7);
  local_8 = 0xffffffff;
  FUN_0061d008();
  ExceptionList = local_14;
  return uVar2;
}


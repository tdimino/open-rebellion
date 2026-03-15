
uint __cdecl
FUN_0053f9f0(int param_1,int *param_2,undefined4 *param_3,undefined4 *param_4,void *param_5)

{
  int iVar1;
  uint uVar2;
  
  if ((param_1 < 800) || (0x323 < param_1)) {
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  if (uVar2 != 0) {
    uVar2 = FUN_0053ff90(param_1,param_2,param_5,&param_1);
    iVar1 = param_1;
    if ((uVar2 != 0) && (param_1 != 0)) {
      FUN_004f26d0((void *)(param_1 + 0x4c),param_3);
      FUN_004f26d0((void *)(iVar1 + 0x50),param_4);
      FUN_0053fcb0(iVar1);
    }
  }
  return uVar2;
}


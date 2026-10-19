
undefined4 __fastcall FUN_00484510(int param_1)

{
  int iVar1;
  void *pvVar2;
  void *pvVar3;
  undefined4 uVar4;
  int iStack_14;
  
  uVar4 = 0;
  iStack_14 = param_1;
  FUN_004f26d0(&iStack_14,(undefined4 *)(param_1 + 0x2c));
  pvVar2 = FUN_00419e40(*(void **)(param_1 + 0x38));
  if (pvVar2 != (void *)0x0) {
    iVar1 = *(int *)(param_1 + 0x40);
    FUN_004f26d0(&iStack_14,(undefined4 *)((int)pvVar2 + 0x28));
    pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (((pvVar3 != (void *)0x0) && ((*(byte *)((int)pvVar3 + 0x30) & 4) != 0)) ||
       ((*(uint *)((int)pvVar2 + 0x38) & 0xf0000000) != 0)) {
      uVar4 = 1;
    }
  }
  return uVar4;
}


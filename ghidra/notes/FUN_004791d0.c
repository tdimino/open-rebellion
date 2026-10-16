
undefined4 __thiscall FUN_004791d0(void *this,uint *param_1)

{
  void *pvVar1;
  int iVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  pvVar1 = FUN_004f5940((void *)((int)this + 0x2c),param_1);
  if (pvVar1 != (void *)0x0) {
    iVar2 = FUN_00476110(*(void **)((int)this + 0x58),param_1);
    if (iVar2 != 0) {
      *(uint *)(iVar2 + 0x38) = *(uint *)(iVar2 + 0x38) | 4;
      *(undefined4 *)(iVar2 + 0x30) = 0;
      *(undefined4 *)(iVar2 + 0x34) = 0;
    }
    FUN_004f58e0((void *)((int)this + 0x2c),param_1);
    uVar3 = 1;
  }
  return uVar3;
}


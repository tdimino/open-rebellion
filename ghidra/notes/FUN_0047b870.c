
undefined4 __thiscall FUN_0047b870(void *this,uint *param_1)

{
  void *pvVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  pvVar1 = FUN_004f5940((void *)((int)this + 0x3c),param_1);
  if (pvVar1 != (void *)0x0) {
    pvVar1 = FUN_004f5940(*(void **)((int)this + 100),param_1);
    if ((pvVar1 != (void *)0x0) && (*(int *)((int)pvVar1 + 0x24) == *(int *)((int)this + 0x18))) {
      *(uint *)((int)pvVar1 + 0x30) = *(uint *)((int)pvVar1 + 0x30) | 0x800000;
      *(undefined4 *)((int)pvVar1 + 0x24) = 0;
      *(undefined4 *)((int)pvVar1 + 0x28) = 0;
      *(undefined4 *)((int)pvVar1 + 0x2c) = 0;
      FUN_004021b0((int)pvVar1);
    }
    FUN_004f58e0((void *)((int)this + 0x3c),param_1);
    uVar2 = 1;
  }
  return uVar2;
}


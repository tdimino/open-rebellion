
undefined4 __thiscall FUN_00475f60(void *this,uint *param_1)

{
  uint uVar1;
  void *pvVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  pvVar2 = FUN_004f5940((void *)((int)this + 0x24),param_1);
  if (pvVar2 != (void *)0x0) {
    pvVar2 = FUN_004f5940((void *)(*(int *)((int)this + 0x4c) + 0x58),param_1);
    if (pvVar2 != (void *)0x0) {
      if (*(int *)((int)pvVar2 + 0x30) == 0) {
        uVar1 = *(uint *)((int)pvVar2 + 0x24);
        *(uint *)((int)pvVar2 + 0x24) = uVar1 | 0x20000;
        *(uint *)((int)pvVar2 + 0x24) = uVar1 & 0xefffffff | 0x20000;
        *(uint *)((int)pvVar2 + 0x24) = uVar1 & 0xecffffff | 0x20000;
        *(undefined4 *)((int)pvVar2 + 0x30) = 0;
        *(undefined4 *)((int)pvVar2 + 0x34) = 0;
      }
      *(int *)((int)this + 0x34) = *(int *)((int)this + 0x34) - *(int *)((int)pvVar2 + 0x7c);
    }
    FUN_004f58e0((void *)((int)this + 0x24),param_1);
    uVar3 = 1;
  }
  return uVar3;
}


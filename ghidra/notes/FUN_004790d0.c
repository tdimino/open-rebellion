
undefined4 __thiscall FUN_004790d0(void *this,uint *param_1)

{
  uint uVar1;
  void *pvVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  pvVar2 = FUN_004f5940((void *)((int)this + 0x24),param_1);
  if (pvVar2 != (void *)0x0) {
    pvVar2 = FUN_004f5940((void *)(*(int *)((int)this + 0x58) + 0x58),param_1);
    if (pvVar2 != (void *)0x0) {
      if (*(int *)((int)pvVar2 + 0x30) == *(int *)((int)this + 0x18)) {
        uVar1 = *(uint *)((int)pvVar2 + 0x24);
        *(uint *)((int)pvVar2 + 0x24) = uVar1 | 0x20000;
        *(uint *)((int)pvVar2 + 0x24) = uVar1 & 0xfcffffff | 0x20000;
        *(undefined4 *)((int)pvVar2 + 0x30) = 0;
        *(undefined4 *)((int)pvVar2 + 0x34) = 0;
      }
      *(int *)((int)this + 0x6c) = *(int *)((int)this + 0x6c) - *(int *)((int)pvVar2 + 0x7c);
      *(int *)((int)this + 0x70) = *(int *)((int)this + 0x70) - *(int *)((int)pvVar2 + 0x80);
      *(int *)((int)this + 0x74) = *(int *)((int)this + 0x74) - *(int *)((int)pvVar2 + 0x6c);
      *(int *)((int)this + 0x78) = *(int *)((int)this + 0x78) - *(int *)((int)pvVar2 + 0x60);
      *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) - *(int *)((int)pvVar2 + 0x48);
      *(int *)((int)this + 0x88) = *(int *)((int)this + 0x88) - *(int *)((int)pvVar2 + 100);
      *(int *)((int)this + 0x8c) = *(int *)((int)this + 0x8c) - *(int *)((int)pvVar2 + 0x58);
      if ((*(byte *)((int)pvVar2 + 0x24) & 0x80) != 0) {
        *(int *)((int)this + 0x7c) = *(int *)((int)this + 0x7c) + -1;
      }
      if ((*(uint *)((int)pvVar2 + 0x24) & 0x400) != 0) {
        *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + -1;
      }
      if ((*(byte *)((int)pvVar2 + 0x24) & 2) != 0) {
        *(int *)((int)this + 0x90) = *(int *)((int)this + 0x90) + -1;
      }
    }
    FUN_004f58e0((void *)((int)this + 0x24),param_1);
    uVar3 = 1;
  }
  return uVar3;
}


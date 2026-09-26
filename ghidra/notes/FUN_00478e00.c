
undefined4 __thiscall FUN_00478e00(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063330b;
  local_c = ExceptionList;
  uVar4 = 0;
  if ((*param_1 >> 0x18 < 0x14) || (0x1b < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    pvVar2 = (void *)FUN_00618b70(0x1c);
    puVar3 = (undefined4 *)0x0;
    local_4 = 0;
    if (pvVar2 != (void *)0x0) {
      puVar3 = FUN_004ecc70(pvVar2,param_1);
    }
    local_4 = 0xffffffff;
    if (puVar3 != (undefined4 *)0x0) {
      pvVar2 = FUN_004f5940((void *)(*(int *)((int)this + 0x58) + 0x58),param_1);
      if (pvVar2 == (void *)0x0) {
        (**(code **)*puVar3)(1);
      }
      else {
        *(uint *)((int)pvVar2 + 0x24) = *(uint *)((int)pvVar2 + 0x24) & 0xfffdffff;
        *(undefined4 *)((int)pvVar2 + 0x30) = *(undefined4 *)((int)this + 0x18);
        *(undefined4 *)((int)pvVar2 + 0x34) = *(undefined4 *)((int)this + 0x44);
        if (0 < *(int *)((int)pvVar2 + 0x7c)) {
          *(int *)((int)this + 0x6c) = *(int *)((int)this + 0x6c) + *(int *)((int)pvVar2 + 0x7c);
          *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x200000;
        }
        if (0 < *(int *)((int)pvVar2 + 0x80)) {
          *(int *)((int)this + 0x70) = *(int *)((int)this + 0x70) + *(int *)((int)pvVar2 + 0x80);
          *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x400000;
        }
        if (0 < *(int *)((int)pvVar2 + 0x6c)) {
          *(int *)((int)this + 0x74) = *(int *)((int)this + 0x74) + *(int *)((int)pvVar2 + 0x6c);
          *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x800000;
        }
        if (0 < *(int *)((int)pvVar2 + 0x60)) {
          *(int *)((int)this + 0x78) = *(int *)((int)this + 0x78) + *(int *)((int)pvVar2 + 0x60);
          *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x1000000;
        }
        if (0 < *(int *)((int)pvVar2 + 0x48)) {
          *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + *(int *)((int)pvVar2 + 0x48);
          *(uint *)((int)this + 100) = *(uint *)((int)this + 100) | 0x2000000;
        }
        *(int *)((int)this + 0x88) = *(int *)((int)this + 0x88) + *(int *)((int)pvVar2 + 100);
        *(int *)((int)this + 0x8c) = *(int *)((int)this + 0x8c) + *(int *)((int)pvVar2 + 0x58);
        if ((*(byte *)((int)pvVar2 + 0x24) & 0x80) != 0) {
          *(int *)((int)this + 0x7c) = *(int *)((int)this + 0x7c) + 1;
        }
        if ((*(uint *)((int)pvVar2 + 0x24) & 0x400) != 0) {
          *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + 1;
        }
        if ((*(byte *)((int)pvVar2 + 0x24) & 2) != 0) {
          *(int *)((int)this + 0x90) = *(int *)((int)this + 0x90) + 1;
        }
        FUN_004f57b0((void *)((int)this + 0x24),puVar3);
        uVar4 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return uVar4;
}


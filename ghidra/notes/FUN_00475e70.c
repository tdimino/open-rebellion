
undefined4 __thiscall FUN_00475e70(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 uVar3;
  uint uVar4;
  undefined4 *puVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632d8b;
  local_c = ExceptionList;
  uVar3 = 0;
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
    puVar5 = (undefined4 *)0x0;
    local_4 = 0;
    if (pvVar2 != (void *)0x0) {
      puVar5 = FUN_004ecc70(pvVar2,param_1);
    }
    local_4 = 0xffffffff;
    if (puVar5 != (undefined4 *)0x0) {
      pvVar2 = FUN_004f5940((void *)(*(int *)((int)this + 0x4c) + 0x58),param_1);
      if (pvVar2 == (void *)0x0) {
        (**(code **)*puVar5)(1);
      }
      else {
        uVar4 = *(uint *)((int)pvVar2 + 0x24) & 0xfffdffff;
        *(uint *)((int)pvVar2 + 0x24) = uVar4;
        *(uint *)((int)pvVar2 + 0x24) = uVar4 | 0x10000000;
        *(undefined4 *)((int)pvVar2 + 0x30) = 0;
        *(undefined4 *)((int)pvVar2 + 0x34) = 0;
        if (0 < *(int *)((int)pvVar2 + 0x7c)) {
          *(int *)((int)this + 0x34) = *(int *)((int)this + 0x34) + *(int *)((int)pvVar2 + 0x7c);
        }
        FUN_004f57b0((void *)((int)this + 0x24),puVar5);
        uVar3 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return uVar3;
}



undefined4 __thiscall FUN_0047b610(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  undefined4 local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063374b;
  local_c = ExceptionList;
  local_18 = 0;
  if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    pvVar2 = FUN_004f5940((void *)((int)this + 0x3c),param_1);
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)FUN_00618b70(0x1c);
      puVar4 = (undefined4 *)0x0;
      local_4 = 0;
      if (pvVar2 != (void *)0x0) {
        puVar4 = FUN_004ecc70(pvVar2,param_1);
      }
      local_4 = 0xffffffff;
      if (puVar4 != (undefined4 *)0x0) {
        pvVar2 = FUN_004f5940(*(void **)((int)this + 100),param_1);
        if (pvVar2 == (void *)0x0) {
          (**(code **)*puVar4)(1);
        }
        else {
          *(uint *)((int)pvVar2 + 0x30) = *(uint *)((int)pvVar2 + 0x30) & 0xff7fffff;
          *(undefined4 *)((int)pvVar2 + 0x24) = *(undefined4 *)((int)this + 0x18);
          *(undefined4 *)((int)pvVar2 + 0x28) = 0;
          uVar3 = (**(code **)(*(int *)this + 0x20))();
          *(undefined4 *)((int)pvVar2 + 0x2c) = uVar3;
          FUN_004021b0((int)pvVar2);
          FUN_004f57b0((void *)((int)this + 0x3c),puVar4);
          *(undefined4 *)((int)this + 0x6c) = 0;
          local_18 = 1;
        }
      }
    }
  }
  ExceptionList = local_c;
  return local_18;
}


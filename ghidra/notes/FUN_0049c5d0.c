
void __thiscall FUN_0049c5d0(void *this,void *param_1)

{
  void *this_00;
  uint *puVar1;
  void *pvVar2;
  uint uVar3;
  void *pvVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar4 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636fd2;
  local_c = ExceptionList;
  if (param_1 != (void *)0x0) {
    ExceptionList = &local_c;
    this_00 = (void *)FUN_00618b70(0x1c);
    local_4 = 0;
    if (this_00 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      puVar1 = FUN_004025b0(pvVar4,(uint *)&param_1);
      local_4 = CONCAT31(local_4._1_3_,1);
      pvVar2 = FUN_004ecc70(this_00,puVar1);
    }
    local_4 = 0;
    if (this_00 != (void *)0x0) {
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    if (pvVar2 != (void *)0x0) {
      uVar3 = *(uint *)((int)pvVar4 + 0x24) >> 6 & 3;
      if (uVar3 == 1) {
        pvVar4 = (void *)((int)this + 0x6c);
      }
      else {
        if (uVar3 != 2) {
          ExceptionList = local_c;
          return;
        }
        pvVar4 = (void *)((int)this + 0x74);
      }
      FUN_004f57b0(pvVar4,pvVar2);
    }
  }
  ExceptionList = local_c;
  return;
}


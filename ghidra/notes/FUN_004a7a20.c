
void __thiscall FUN_004a7a20(void *this,void *param_1)

{
  bool bVar1;
  void *pvVar2;
  void *this_00;
  undefined4 *puVar3;
  int *piVar4;
  undefined1 local_24 [4];
  void *local_20;
  undefined4 local_1c [2];
  int *local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637faa;
  pvStack_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &pvStack_c;
  FUN_0060a790(local_1c,1);
  local_4 = 0;
  if (*(void **)((int)this + 0x164) != (void *)0x0) {
    FUN_00609410(*(void **)((int)this + 0x164),local_1c);
    for (piVar4 = local_14; piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 0xc))()) {
      pvVar2 = (void *)FUN_0060a860(*(void **)((int)this + 0x168),piVar4[3]);
      this_00 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 1;
      local_20 = this_00;
      if (this_00 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        puVar3 = FUN_0042d170(pvVar2,local_24);
        bVar1 = true;
        local_4 = CONCAT31(local_4._1_3_,2);
        pvVar2 = FUN_004f5b10(this_00,puVar3,0);
      }
      local_4 = 1;
      if (bVar1) {
        bVar1 = false;
        FUN_00619730();
      }
      local_4 = local_4 & 0xffffff00;
      FUN_004f57b0(param_1,pvVar2);
    }
  }
  local_4 = 0xffffffff;
  FUN_0060a810(local_1c);
  ExceptionList = pvStack_c;
  return;
}



void __thiscall FUN_004030d0(void *this,void *param_1)

{
  void *pvVar1;
  undefined1 uVar2;
  undefined4 *this_00;
  uint *puVar3;
  void *pvVar4;
  void *pvStack_1c;
  undefined1 auStack_18 [4];
  undefined4 local_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062942b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_14);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  FUN_005f4d90(param_1,(int)this + 8);
  FUN_005f4d90(param_1,(int)this + 0x18);
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&pvStack_1c);
  param_1 = (void *)0x0;
  if (pvStack_1c != (void *)0x0) {
    do {
      puStack_10 = (undefined4 *)FUN_00618b70(0x68);
      local_4._0_1_ = 1;
      if (puStack_10 == (undefined4 *)0x0) {
        this_00 = (undefined4 *)0x0;
      }
      else {
        this_00 = FUN_00401c70(puStack_10);
      }
      local_4._0_1_ = 0;
      uVar2 = (undefined1)local_4;
      local_4._0_1_ = 0;
      if (this_00 != (undefined4 *)0x0) {
        FUN_00402070(this_00,pvVar1,*(undefined4 *)((int)this + 0x10));
        puVar3 = FUN_00403040(this_00,auStack_18);
        local_4._0_1_ = 2;
        pvVar4 = FUN_004f5940(this,puVar3);
        local_4._0_1_ = 0;
        FUN_00619730();
        uVar2 = (undefined1)local_4;
        if (pvVar4 == (void *)0x0) {
          FUN_004f57b0(this,this_00);
          FUN_004021b0((int)this_00);
          uVar2 = (undefined1)local_4;
        }
      }
      local_4._0_1_ = uVar2;
      param_1 = (void *)((int)param_1 + 1);
    } while (param_1 < pvStack_1c);
  }
  FUN_005f4d90(pvVar1,(int)this + 0xc);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}


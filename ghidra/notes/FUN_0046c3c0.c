
void __thiscall FUN_0046c3c0(void *this,uint param_1,void *param_2)

{
  undefined1 uVar1;
  void **ppvVar2;
  int iVar3;
  void *pvVar4;
  uint *puVar5;
  void *this_00;
  int *piVar6;
  undefined4 *puVar7;
  int iVar8;
  void *this_01;
  short sVar9;
  void *local_64;
  void *local_60;
  undefined1 auStack_5c [4];
  undefined1 auStack_58 [4];
  void *local_54;
  int iStack_50;
  undefined4 local_4c [2];
  undefined4 local_44 [2];
  undefined4 local_3c [2];
  undefined4 auStack_34 [2];
  undefined4 local_2c [2];
  undefined4 auStack_24 [2];
  undefined4 local_1c [2];
  int *piStack_14;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631fd8;
  local_c = ExceptionList;
  sVar9 = (short)param_1;
  ppvVar2 = &local_c;
  local_54 = this;
  switch(param_1 & 0xffff) {
  case 0x66:
    ExceptionList = &local_c;
    iVar3 = FUN_0060c960(*(int *)((int)this + 0x128));
    FUN_004f26d0((void *)(*(int *)((int)this + 0x120) + 0x4c),*(undefined4 **)(iVar3 + 0x54));
    FUN_0041ce20(*(int **)((int)this + 0x120),0);
    *(undefined4 *)((int)this + 0x120) = 0;
    ppvVar2 = ExceptionList;
  case 100:
  case 0x65:
    ExceptionList = ppvVar2;
    if (*(undefined4 **)((int)this + 0x120) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0x120))(1);
      *(undefined4 *)((int)this + 0x120) = 0;
    }
    (**(code **)(*(int *)this + 0x30))();
    break;
  case 0x67:
    ExceptionList = &local_c;
    FUN_0042dbe0(&param_1);
    local_4 = 0;
    FUN_004ece30(&param_2);
    local_4._0_1_ = 1;
    iVar3 = FUN_0060c960(*(int *)((int)this + 0x128));
    FUN_004f26d0(&param_1,*(undefined4 **)(iVar3 + 0x54));
    FUN_0041d6b0(&param_2,&param_1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    break;
  case 0x68:
    if (*(int *)((int)this + 0x128) != 0) {
      ExceptionList = &local_c;
      FUN_0060c1f0(*(int *)((int)this + 0x128));
    }
    break;
  case 0x96:
    ExceptionList = &local_c;
    FUN_0046c8a0(this,(uint)param_2 >> 0x10);
    break;
  case 200:
  case 0xc9:
    if ((uint)param_2 >> 0x10 != 0x309) {
      return;
    }
    param_1 = (sVar9 != 200) + 0xca;
    sVar9 = (short)param_1;
  case 0xca:
  case 0xcb:
    ExceptionList = &local_c;
    FUN_0060a790(local_1c,1);
    local_4 = 2;
    FUN_004f4340(local_4c);
    local_4._0_1_ = 3;
    FUN_004f4340(local_44);
    local_4._0_1_ = 4;
    uVar1 = (undefined1)local_4;
    local_4._0_1_ = 4;
    if (sVar9 == 0xca) {
      param_2 = *(void **)((int)this + 0x130);
      local_60 = *(void **)((int)this + 0x138);
      local_64 = *(void **)((int)this + 0x134);
      this_01 = *(void **)((int)this + 300);
      pvVar4 = FUN_0046c850(*(void **)((int)this + 0x120),local_3c);
      local_4._0_1_ = 5;
      FUN_004f43b0(local_4c,pvVar4);
      local_4._0_1_ = 4;
      FUN_004f4380(local_3c);
      pvVar4 = FUN_0046c880(*(void **)((int)this + 0x120),auStack_34);
      local_4._0_1_ = 6;
      FUN_004f43b0(local_44,pvVar4);
      puVar7 = auStack_34;
    }
    else {
      param_2 = *(void **)((int)this + 0x138);
      local_64 = *(void **)((int)this + 300);
      local_60 = *(void **)((int)this + 0x130);
      this_01 = *(void **)((int)this + 0x134);
      local_4._0_1_ = uVar1;
      pvVar4 = FUN_0046c880(*(void **)((int)this + 0x120),local_2c);
      local_4._0_1_ = 7;
      FUN_004f43b0(local_4c,pvVar4);
      local_4._0_1_ = 4;
      FUN_004f4380(local_2c);
      pvVar4 = FUN_0046c850(*(void **)((int)this + 0x120),auStack_24);
      local_4._0_1_ = 8;
      FUN_004f43b0(local_44,pvVar4);
      puVar7 = auStack_24;
    }
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_004f4380(puVar7);
    FUN_00609410(param_2,local_1c);
    iVar3 = 0;
    if (piStack_14 != (int *)0x0) {
      iStack_50 = 1;
      piVar6 = piStack_14;
      do {
        pvVar4 = (void *)FUN_0060a860(this_01,piVar6[3]);
        puVar5 = FUN_0042d170(pvVar4,auStack_5c);
        local_4._0_1_ = 9;
        this_00 = FUN_004f5940(local_4c,puVar5);
        local_4._0_1_ = 4;
        FUN_00619730();
        puVar5 = FUN_00403040(this_00,auStack_58);
        local_4._0_1_ = 10;
        FUN_004f5910(local_4c,puVar5);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00619730();
        FUN_004f57b0(local_44,this_00);
        *(uint *)((int)pvVar4 + 0x3c) = *(uint *)((int)pvVar4 + 0x3c) & 0xfffffffe;
        FUN_005f5ac0(this_01,(int)pvVar4);
        FUN_005f59f0(local_64,(int)pvVar4);
        piVar6 = (int *)(**(code **)(*piVar6 + 0xc))();
        iVar3 = iStack_50;
        this = local_54;
      } while (piVar6 != (int *)0x0);
    }
    if ((short)param_1 == 0xca) {
      FUN_004f43b0((void *)(*(int *)((int)this + 0x120) + 0x58),local_44);
      puVar7 = local_4c;
      iVar8 = **(int **)((int)this + 0x120);
    }
    else {
      FUN_004f43b0((void *)(*(int *)((int)this + 0x120) + 0x58),local_4c);
      puVar7 = local_44;
      iVar8 = **(int **)((int)this + 0x120);
    }
    (**(code **)(iVar8 + 0x24))(puVar7);
    if (iVar3 != 0) {
      FUN_0060a280(param_2);
      FUN_0060a280(local_60);
    }
    local_4._0_1_ = 3;
    FUN_004f4380(local_44);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_004f4380(local_4c);
    local_4 = 0xffffffff;
    FUN_0060a810(local_1c);
  }
  ExceptionList = local_c;
  return;
}


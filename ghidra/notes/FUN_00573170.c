
int __thiscall
FUN_00573170(void *this,void *param_1,uint param_2,uint param_3,uint param_4,void *param_5)

{
  uint uVar1;
  int iVar2;
  void *this_00;
  bool bVar3;
  int *piVar4;
  uint *puVar5;
  uint *puVar6;
  uint uVar7;
  undefined3 extraout_var;
  int iVar8;
  undefined4 *this_01;
  uint uVar9;
  int iVar10;
  int local_2c;
  uint local_24;
  uint uStack_20;
  undefined4 local_1c [2];
  uint local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064c5c8;
  pvStack_c = ExceptionList;
  local_2c = 1;
  ExceptionList = &pvStack_c;
  FUN_00520570(&local_24);
  local_4 = 0;
  local_4._1_3_ = 0;
  if ((param_3 == 0) || (param_4 == 0)) {
    if (param_3 == 0) {
      if (param_4 == 0) goto LAB_0057324e;
      local_14 = 0x92;
      local_10 = 0x94;
      local_4._0_1_ = 3;
      FUN_00520580(&local_24,&local_14);
      local_4 = (uint)local_4._1_3_ << 8;
    }
    else {
      local_14 = 0x90;
      local_10 = 0x92;
      local_4._0_1_ = 2;
      FUN_00520580(&local_24,&local_14);
      local_4 = (uint)local_4._1_3_ << 8;
    }
  }
  else {
    local_14 = 0x90;
    local_10 = 0x98;
    local_4._0_1_ = 1;
    FUN_00520580(&local_24,&local_14);
    local_4 = (uint)local_4._1_3_ << 8;
  }
  FUN_00619730();
LAB_0057324e:
  uVar9 = 0;
  param_3 = 0;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 4;
  piVar4 = (int *)FUN_00505ef0();
  this_00 = param_1;
  iVar2 = local_2c;
  uVar7 = param_3;
  while (piVar4 != (int *)0x0) {
    local_2c = iVar2;
    if (((piVar4[9] ^ *(uint *)((int)this_00 + 0x24)) & 0xc0) == 0) {
      puVar5 = FUN_004025b0(this_00,&param_4);
      local_4._0_1_ = 6;
      puVar6 = FUN_004025b0(piVar4,&param_3);
      uVar7 = *puVar6;
      uVar1 = *puVar5;
      FUN_00619730();
      local_4._0_1_ = 4;
      FUN_00619730();
      if (((uVar7 != uVar1) && (uVar7 = (**(code **)(*piVar4 + 4))(), local_24 <= uVar7)) &&
         (uVar7 < uStack_20)) {
        uVar9 = uVar9 + 1;
        puVar5 = FUN_004025b0(piVar4,(uint *)&param_1);
        local_4._0_1_ = 7;
        bVar3 = FUN_004f44b0(local_1c,puVar5,0);
        if ((CONCAT31(extraout_var,bVar3) == 0) || (local_2c = 1, iVar2 == 0)) {
          local_2c = 0;
        }
        local_4._0_1_ = 4;
        FUN_00619730();
      }
    }
    local_14 = 0x90;
    local_10 = 0x98;
    local_4._0_1_ = 5;
    piVar4 = (int *)FUN_004f6010(piVar4,&local_14,1);
    local_4._0_1_ = 4;
    FUN_00619730();
    iVar2 = local_2c;
    uVar7 = uVar9;
  }
  param_3 = uVar7;
  if ((int)uVar9 < (int)param_2) {
    param_2 = uVar9;
  }
  do {
    do {
      if (param_2 == 0) {
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_004f4380(local_1c);
        local_4 = 0xffffffff;
        FUN_00619730();
        ExceptionList = pvStack_c;
        return iVar2;
      }
      uVar7 = uVar9 - 1;
      iVar8 = FUN_0053e290(uVar7);
      iVar10 = 0;
      this_01 = (undefined4 *)thunk_FUN_005f5060((int)local_1c);
      uVar9 = param_3;
    } while (this_01 == (undefined4 *)0x0);
    do {
      if (iVar8 <= iVar10) break;
      this_01 = (undefined4 *)this_01[4];
      iVar10 = iVar10 + 1;
    } while (this_01 != (undefined4 *)0x0);
    if (this_01 != (undefined4 *)0x0) {
      piVar4 = FUN_00403040(this_01,&param_4);
      local_4._0_1_ = 8;
      iVar8 = FUN_00572e30(this,piVar4,param_5);
      if ((iVar8 == 0) || (local_2c = 1, iVar2 == 0)) {
        local_2c = 0;
      }
      local_4._0_1_ = 4;
      FUN_00619730();
      FUN_005f4fa0(local_1c,(int)this_01);
      (**(code **)*this_01)(1);
      param_2 = param_2 - 1;
      uVar9 = uVar7;
      iVar2 = local_2c;
      param_3 = uVar7;
    }
  } while( true );
}


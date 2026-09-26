
uint __thiscall FUN_0057e0d0(void *this,int *param_1)

{
  int *this_00;
  int iVar1;
  uint *puVar2;
  uint uVar3;
  uint uVar4;
  int *local_64;
  undefined4 uStack_60;
  uint local_5c [11];
  undefined4 auStack_30 [9];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_0064e2e0;
  pvStack_c = ExceptionList;
  uVar4 = 1;
  local_64 = (int *)0x0;
  local_5c[0] = 1;
  local_5c[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,local_5c);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (this_00[1] == -1) {
    local_5c[1] = 0x12;
    local_5c[0] = 1;
    local_4 = 1;
    uVar4 = FUN_005057f0((uint *)((int)this + 0x3c),&local_64,local_5c,this_00);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  param_1 = (int *)0x0;
  local_5c[2] = 1;
  local_5c[3] = 0xffffffff;
  local_4 = 2;
  if (uVar4 == 0) goto LAB_0057e354;
  if (this_00[1] == -1) {
    local_5c[0] = 1;
    local_5c[1] = 0x22;
    local_4._0_1_ = 3;
    local_4._1_3_ = 0;
    uVar4 = FUN_00504e60((uint *)((int)this + 0x48),&param_1,local_5c,local_5c + 2);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
    if (uVar4 == 0) goto LAB_0057e354;
    if (param_1 == (int *)0x0) {
      FUN_00520580(this_00,local_5c + 2);
    }
  }
  if ((uVar4 == 0) || (this_00[1] != -1)) goto LAB_0057e354;
  uVar4 = (**(code **)(*local_64 + 0x1e4))((int *)((int)this + 0x40),(int)this + 0x50,this_00);
  if ((uVar4 == 0) || ((this_00[1] != -1 || (local_64[0x16] == 0)))) goto LAB_0057e354;
  FUN_00551060(auStack_30);
  local_4._0_1_ = 4;
  if (this_00[1] == -1) {
    uStack_60 = 3;
    iVar1 = FUN_00551190(auStack_30,(int *)((int)this + 0x40),(uint)local_64[9] >> 6 & 3,&uStack_60)
    ;
    uVar4 = 0;
    if (iVar1 != 0) {
      puVar2 = FUN_004025b0(param_1,local_5c);
      local_4._0_1_ = 5;
      uVar3 = FUN_005513a0(auStack_30,puVar2);
      local_4._0_1_ = 4;
      FUN_00619730();
      uVar4 = 0;
      if (uVar3 != 0) {
        uVar4 = FUN_00552210(auStack_30,local_64,this_00);
      }
    }
  }
  iVar1 = thunk_FUN_005f5060((int)this + 0x50);
  if (uVar4 == 0) {
LAB_0057e317:
    local_5c[0] = 1;
    local_5c[1] = 1;
    local_4._0_1_ = 7;
    FUN_00520580(this_00,local_5c);
    local_4._0_1_ = 4;
    FUN_00619730();
  }
  else {
    do {
      if ((this_00[1] != -1) || (iVar1 == 0)) break;
      FUN_0054fa40(local_5c + 4);
      local_4._0_1_ = 6;
      if ((uVar4 != 0) && (this_00[1] == -1)) {
        uVar4 = FUN_00551ee0(auStack_30,(uint *)(iVar1 + 0x1c),(uint *)(iVar1 + 0x18),this_00,
                             local_5c + 4);
      }
      local_4._0_1_ = 4;
      FUN_0054fa90(local_5c + 4);
      iVar1 = *(int *)(iVar1 + 0x10);
    } while (uVar4 != 0);
    if (uVar4 == 0) goto LAB_0057e317;
  }
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005510b0(auStack_30);
LAB_0057e354:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar4;
}


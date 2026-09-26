
uint FUN_00546fa0(void *param_1)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  uint *puVar3;
  void *this;
  int iVar4;
  uint uVar5;
  undefined3 extraout_var_00;
  uint uVar6;
  int *this_00;
  uint local_24;
  int local_20 [4];
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006477e0;
  local_c = ExceptionList;
  uVar6 = 1;
  ExceptionList = &local_c;
  bVar2 = FUN_0053e2f0(DAT_006bb5b8);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    FUN_004ece30(&local_24);
    local_4 = 0;
    uVar6 = FUN_005435f0(local_20,local_20 + 2,&local_24);
    if ((local_20[0] != 0) && (local_20[2] != 0)) {
      puVar3 = FUN_004ece40(&local_24);
      if ((puVar3 != (uint *)0x0) &&
         (((*(byte *)(local_20[0] + 0xac) & 1) == 0 && ((*(byte *)(local_20[0] + 0x50) & 1) != 0))))
      {
        this = (void *)thunk_FUN_00506ed0();
        pvVar1 = param_1;
        this_00 = (int *)0x0;
        if (this != (void *)0x0) {
          param_1 = (void *)0x3c000009;
          local_4._0_1_ = 1;
          this_00 = FUN_004f7d50(this,(uint *)&param_1,*(uint *)((int)this + 0x24) >> 6 & 3,pvVar1,1
                                 ,1);
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
        }
        iVar4 = thunk_FUN_00506e60();
        uVar6 = (uint)(iVar4 != 0);
        if (iVar4 != 0) {
          param_1 = (void *)0x33000243;
          local_4._0_1_ = 2;
          uVar5 = FUN_00556c70((uint *)&param_1,2,(int)pvVar1);
          if ((uVar5 == 0) || (uVar6 == 0)) {
            uVar6 = 0;
          }
          else {
            uVar6 = 1;
          }
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
        }
        if (this_00 != (int *)0x0) {
          puVar3 = FUN_004025b0(this_00,&local_10);
          local_20[1] = 0x65000083;
          param_1 = (void *)0x33000243;
          local_4._0_1_ = 5;
          bVar2 = FUN_005479c0((uint *)&param_1,(uint *)(local_20 + 1),puVar3,(uint)pvVar1);
          if ((CONCAT31(extraout_var_00,bVar2) == 0) || (uVar6 == 0)) {
            uVar6 = 0;
          }
          else {
            uVar6 = 1;
          }
          local_4._0_1_ = 4;
          FUN_00619730();
          local_4._0_1_ = 3;
          FUN_00619730();
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
        }
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar6;
}


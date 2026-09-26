
void __fastcall FUN_00402720(void *param_1)

{
  int iVar1;
  bool bVar2;
  int *piVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  undefined3 extraout_var;
  undefined1 local_68 [4];
  undefined1 auStack_64 [4];
  undefined1 auStack_60 [4];
  undefined1 auStack_5c [4];
  undefined1 auStack_58 [4];
  undefined1 auStack_54 [4];
  undefined1 auStack_50 [4];
  undefined1 auStack_4c [4];
  undefined1 auStack_48 [4];
  undefined1 auStack_44 [4];
  undefined1 auStack_40 [4];
  undefined1 auStack_3c [4];
  undefined1 auStack_38 [4];
  undefined1 auStack_34 [4];
  undefined1 auStack_30 [4];
  void *local_2c;
  int *local_28;
  undefined4 local_24 [2];
  int local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062935a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004f3f70(local_1c);
  local_4 = 0;
  FUN_004f4340(local_24);
  local_4._0_1_ = 1;
  piVar3 = (int *)FUN_004f5cd0(0x240);
  local_28 = piVar3;
  if (piVar3 != (int *)0x0) {
    piVar3[8] = *(int *)((int)param_1 + 0x1c);
    pvVar4 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 2;
    local_2c = pvVar4;
    if (pvVar4 == (void *)0x0) {
      pvVar6 = (void *)0x0;
    }
    else {
      puVar5 = FUN_00403040(param_1,local_68);
      local_4 = CONCAT31(local_4._1_3_,3);
      pvVar6 = FUN_004f5b10(pvVar4,puVar5,0);
    }
    local_4 = 2;
    if (pvVar4 != (void *)0x0) {
      FUN_00619730();
    }
    local_4._0_1_ = 1;
    if (pvVar6 != (void *)0x0) {
      FUN_004f57b0(local_24,pvVar6);
      (**(code **)(*piVar3 + 0x24))(local_24);
      bVar2 = FUN_004f52c0(piVar3,local_1c);
      if (CONCAT31(extraout_var,bVar2) != 0) {
        pvVar4 = (void *)thunk_FUN_005f5060((int)local_1c);
        for (; pvVar4 != (void *)0x0; pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
          local_4._0_1_ = 4;
          piVar3 = FUN_00559950(pvVar4,local_68);
          iVar1 = *piVar3;
          FUN_00619730();
          local_4._0_1_ = 1;
          FUN_00619730();
          if (iVar1 == 0x51000010) {
            *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 1;
          }
          else {
            local_4._0_1_ = 5;
            piVar3 = FUN_00559950(pvVar4,auStack_64);
            iVar1 = *piVar3;
            FUN_00619730();
            local_4._0_1_ = 1;
            FUN_00619730();
            if (iVar1 == 0x61000011) {
              *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 2;
            }
            else {
              local_4._0_1_ = 6;
              piVar3 = FUN_00559950(pvVar4,auStack_60);
              iVar1 = *piVar3;
              FUN_00619730();
              local_4._0_1_ = 1;
              FUN_00619730();
              if (iVar1 == 0x69000012) {
                *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 4;
              }
              else {
                local_4._0_1_ = 7;
                piVar3 = FUN_00559950(pvVar4,auStack_5c);
                iVar1 = *piVar3;
                FUN_00619730();
                local_4._0_1_ = 1;
                FUN_00619730();
                if (iVar1 == 0x52000013) {
                  *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 8;
                }
                else {
                  local_4._0_1_ = 8;
                  piVar3 = FUN_00559950(pvVar4,auStack_58);
                  iVar1 = *piVar3;
                  FUN_00619730();
                  local_4._0_1_ = 1;
                  FUN_00619730();
                  if (iVar1 == 0x62000017) {
                    *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 0x10;
                  }
                  else {
                    local_4._0_1_ = 9;
                    piVar3 = FUN_00559950(pvVar4,auStack_54);
                    iVar1 = *piVar3;
                    FUN_00619730();
                    local_4._0_1_ = 1;
                    FUN_00619730();
                    if (iVar1 == 0x57000080) {
                      *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 0x20;
                    }
                    else {
                      local_4._0_1_ = 10;
                      piVar3 = FUN_00559950(pvVar4,auStack_50);
                      iVar1 = *piVar3;
                      FUN_00619730();
                      local_4._0_1_ = 1;
                      FUN_00619730();
                      if (iVar1 == 0x63000081) {
                        *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 0x40;
                      }
                      else {
                        local_4._0_1_ = 0xb;
                        piVar3 = FUN_00559950(pvVar4,auStack_4c);
                        iVar1 = *piVar3;
                        FUN_00619730();
                        local_4._0_1_ = 1;
                        FUN_00619730();
                        if (iVar1 == 0x6a000041) {
                          *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 0x80;
                        }
                        else {
                          local_4._0_1_ = 0xc;
                          piVar3 = FUN_00559950(pvVar4,auStack_48);
                          iVar1 = *piVar3;
                          FUN_00619730();
                          local_4._0_1_ = 1;
                          FUN_00619730();
                          if (iVar1 == 0x56000040) {
                            *(uint *)((int)param_1 + 0x34) = *(uint *)((int)param_1 + 0x34) | 0x100;
                          }
                          else {
                            local_4._0_1_ = 0xd;
                            piVar3 = FUN_00559950(pvVar4,auStack_44);
                            iVar1 = *piVar3;
                            FUN_00619730();
                            local_4._0_1_ = 1;
                            FUN_00619730();
                            if (iVar1 == 0x54000015) {
                              *(uint *)((int)param_1 + 0x34) =
                                   *(uint *)((int)param_1 + 0x34) | 0x200;
                            }
                            else {
                              local_4._0_1_ = 0xe;
                              piVar3 = FUN_00559950(pvVar4,auStack_40);
                              iVar1 = *piVar3;
                              FUN_00619730();
                              local_4._0_1_ = 1;
                              FUN_00619730();
                              if (iVar1 == 0x53000020) {
                                *(uint *)((int)param_1 + 0x34) =
                                     *(uint *)((int)param_1 + 0x34) | 0x800;
                              }
                              else {
                                local_4._0_1_ = 0xf;
                                piVar3 = FUN_00559950(pvVar4,auStack_3c);
                                iVar1 = *piVar3;
                                FUN_00619730();
                                local_4._0_1_ = 1;
                                FUN_00619730();
                                if (iVar1 == 0x53000022) {
                                  *(uint *)((int)param_1 + 0x34) =
                                       *(uint *)((int)param_1 + 0x34) | 0x1000;
                                }
                                else {
                                  local_4._0_1_ = 0x10;
                                  piVar3 = FUN_00559950(pvVar4,auStack_38);
                                  iVar1 = *piVar3;
                                  FUN_00619730();
                                  local_4._0_1_ = 1;
                                  FUN_00619730();
                                  if (iVar1 == 0x53000021) {
                                    *(uint *)((int)param_1 + 0x34) =
                                         *(uint *)((int)param_1 + 0x34) | 0x2000;
                                  }
                                  else {
                                    local_4._0_1_ = 0x11;
                                    piVar3 = FUN_00559950(pvVar4,auStack_34);
                                    iVar1 = *piVar3;
                                    FUN_00619730();
                                    local_4._0_1_ = 1;
                                    FUN_00619730();
                                    if (iVar1 == 0x55000016) {
                                      *(uint *)((int)param_1 + 0x34) =
                                           *(uint *)((int)param_1 + 0x34) | 0x400;
                                    }
                                    else {
                                      local_2c = (void *)0x58000042;
                                      local_4._0_1_ = 0x12;
                                      FUN_00559950(pvVar4,auStack_30);
                                      FUN_00619730();
                                      local_4._0_1_ = 1;
                                      FUN_00619730();
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          piVar3 = local_28;
        }
      }
    }
    (**(code **)*piVar3)(1);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_24);
  local_4 = 0xffffffff;
  FUN_004f3fb0(local_1c);
  ExceptionList = pvStack_c;
  return;
}


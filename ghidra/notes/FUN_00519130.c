
uint __fastcall FUN_00519130(int *param_1)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  undefined4 uVar4;
  void *pvVar5;
  undefined3 extraout_var;
  int iVar6;
  undefined4 uVar7;
  uint local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642bd8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_004f9700(param_1);
  if (uVar2 != 0) {
    if ((param_1[9] & 0x30U) == 0) {
      if ((((((param_1[0x2c] == 0) || (param_1[0x2d] == 0)) || (param_1[0x2e] == 0)) ||
           (((param_1[0x2f] == 0 || (param_1[0x30] == 0)) ||
            ((param_1[0x31] == 0 || ((param_1[0x32] == 0 || (param_1[0x33] == 0)))))))) ||
          (param_1[0x34] == 0)) ||
         (((param_1[0x35] == 0 || (param_1[0x36] == 0)) || (param_1[0x37] == 0)))) {
        uVar2 = 0;
      }
      else {
        uVar2 = 1;
      }
    }
    if (uVar2 != 0) {
      uVar2 = FUN_00519350(param_1);
      if ((uVar2 != 0) && (uVar2 = FUN_00519470(param_1), uVar2 != 0)) {
        iVar6 = 0;
        do {
          if (2 < iVar6) {
            ExceptionList = local_c;
            return uVar2;
          }
          local_50 = 0xf2000003;
          local_4 = 0;
          iVar3 = FUN_0053efd0(&local_50,iVar6);
          local_4 = 0xffffffff;
          FUN_00619730();
          uVar2 = (uint)(iVar3 != 0);
          if (uVar2 != 0) {
            FUN_004fcd00(local_4c,iVar3,4);
            local_4 = 1;
            FUN_00513120((int)local_4c);
            do {
              if (local_30 == 0) break;
              iVar3 = FUN_0052bed0((int)local_4c);
              local_50 = *(uint *)(iVar3 + 0x50) >> 6 & 1;
              if (local_50 == 0) {
                uVar7 = 4;
                uVar4 = FUN_0052bed0((int)local_4c);
                pvVar5 = FUN_004fcd00(local_2c,uVar4,uVar7);
                local_4._0_1_ = 2;
                bVar1 = FUN_005131b0((int)pvVar5);
                local_4 = CONCAT31(local_4._1_3_,1);
                FUN_004fcd80(local_2c);
                if (CONCAT31(extraout_var,bVar1) != 0) {
                  iVar3 = FUN_0052bed0((int)local_4c);
                  local_50 = *(uint *)(iVar3 + 0x50) >> 1 & 1;
                  if (local_50 == 0) {
                    uVar2 = (uint)(iVar6 != 0);
                  }
                  iVar3 = FUN_0052bed0((int)local_4c);
                  local_50 = *(uint *)(iVar3 + 0x50) >> 3 & 1;
                  if (local_50 != 0) {
                    uVar2 = 0;
                  }
                }
              }
              else {
                uVar2 = (uint)(iVar6 != 0);
              }
              FUN_005130d0((int)local_4c);
            } while (uVar2 != 0);
            local_4 = 0xffffffff;
            FUN_004fcd80(local_4c);
          }
          iVar6 = iVar6 + 1;
        } while (uVar2 != 0);
      }
    }
  }
  ExceptionList = local_c;
  return 0;
}


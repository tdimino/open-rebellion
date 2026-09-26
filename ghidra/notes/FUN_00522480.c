
int __thiscall FUN_00522480(void *this,undefined4 param_1)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  int iVar3;
  int iVar4;
  uint uVar5;
  int local_bc;
  undefined4 local_b8;
  int local_b4;
  int local_b0;
  undefined4 local_ac;
  int local_a8;
  undefined4 auStack_a4 [4];
  undefined1 local_94 [4];
  code *pcStack_90;
  uint local_8c [5];
  undefined4 local_78 [11];
  int local_4c;
  undefined4 auStack_48 [15];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_006439bc;
  pvStack_c = ExceptionList;
  iVar4 = 1;
  local_bc = 0;
  local_b8 = 1;
  local_b4 = -1;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  bVar1 = FUN_00520e40((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) goto LAB_0052279e;
  uVar5 = *(uint *)((int)this + 0xa4) >> 1 & 1;
  FUN_00525c60(local_78,this);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00525930((int)local_78);
  while (local_4c != 0) {
    if (uVar5 != 0) goto LAB_0052256f;
    uVar5 = (uint)((*(byte *)(local_4c + 0x78) & 0xc) == 0);
    FUN_005258f0((int)local_78);
  }
  if (uVar5 == 0) {
    local_bc = 5;
    local_b0 = 0x40;
    local_ac = 0x91;
    local_4._1_3_ = (undefined3)(local_4 >> 8);
    local_4._0_1_ = 2;
    FUN_00520580(&local_b8,&local_b0);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
LAB_0052256f:
  if (local_b4 == -1) {
    local_a8 = 0;
    local_b0 = 0;
    iVar4 = FUN_00520af0((int)this);
    if (iVar4 == 0) {
      bVar1 = FUN_00521160(this,&local_a8);
      bVar2 = FUN_005211c0(this,&local_b0);
      if ((CONCAT31(extraout_var_03,bVar2) == 0) || (CONCAT31(extraout_var_02,bVar1) == 0))
      goto LAB_005225eb;
      iVar4 = 1;
    }
    else {
      bVar1 = FUN_00521030(this,&local_a8);
      bVar2 = FUN_00521050(this,&local_b0);
      if ((CONCAT31(extraout_var_01,bVar2) == 0) || (CONCAT31(extraout_var_00,bVar1) == 0)) {
LAB_005225eb:
        iVar4 = 0;
      }
      else {
        iVar4 = 1;
      }
    }
    FUN_00568fe0(local_8c);
    local_4._0_1_ = 3;
    if (local_b4 == -1) {
      iVar3 = FUN_005830a0((int)this + 0x84,(int)this + 0x8c,(int)this + 0x94,local_8c,&local_b8);
      if ((iVar3 == 0) || (iVar4 == 0)) {
        iVar4 = 0;
      }
      else {
        iVar4 = 1;
      }
      if (local_b4 == -1) {
        iVar3 = (**(code **)(*(int *)this + 0x1bc))(local_94);
        if ((iVar3 == 0) || (iVar4 == 0)) {
          iVar4 = 0;
        }
        else {
          iVar4 = 1;
        }
        if (local_b4 == -1) {
          FUN_004f3f70(auStack_a4);
          local_4._0_1_ = 4;
          FUN_00582b90(auStack_48,0,1,1,*(uint *)((int)this + 0x24) >> 6 & 3,
                       *(undefined4 *)((int)this + 0x2c),auStack_a4,auStack_a4,auStack_a4,
                       (int)this + 0x84,(int)this + 0x8c,(int)this + 0x94,local_8c,local_b0,local_a8
                      );
          local_4._0_1_ = 5;
          if (pcStack_90 != (code *)0x0) {
            iVar3 = (*pcStack_90)(auStack_48,&local_b8,&local_bc);
            if ((iVar3 == 0) || (iVar4 == 0)) {
              iVar4 = 0;
            }
            else {
              iVar4 = 1;
            }
          }
          local_4._0_1_ = 4;
          FUN_00582c20(auStack_48);
          local_4._0_1_ = 3;
          FUN_004f3fb0(auStack_a4);
        }
      }
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  if (local_bc == 0) {
    if (*(int *)((int)this + 0x68) == 0xb) {
      local_bc = 1;
    }
    if (local_bc != 0) goto LAB_00522768;
  }
  else {
LAB_00522768:
    iVar3 = (**(code **)(*(int *)this + 0x1dc))(local_bc,param_1);
    if ((iVar3 == 0) || (iVar4 == 0)) {
      iVar4 = 0;
    }
    else {
      iVar4 = 1;
    }
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00525d00(local_78);
LAB_0052279e:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar4;
}


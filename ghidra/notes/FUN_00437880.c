
void * __thiscall
FUN_00437880(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,int param_6,undefined4 param_7,undefined4 param_8,int param_9,int param_10,
            uint param_11,uint param_12,undefined4 param_13)

{
  int iVar1;
  int iVar2;
  LPBYTE pBVar3;
  void *this_00;
  void *pvVar4;
  void *pvVar5;
  uint *this_01;
  uint *this_02;
  uint *this_03;
  uint uVar6;
  char *pcVar7;
  undefined1 uVar8;
  undefined4 local_28 [3];
  int local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d633;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606380(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10
               ,param_11,param_12,0,0);
  local_4 = 0;
  FUN_0060a790((void *)((int)this + 0x118),1);
  local_4._0_1_ = 1;
  FUN_0042dbe0((undefined4 *)((int)this + 0x150));
  local_4 = CONCAT31(local_4._1_3_,2);
  *(undefined ***)this = &PTR_FUN_00658db8;
  param_10 = 0;
  *(undefined4 *)((int)this + 0x144) = param_13;
  *(undefined4 *)((int)this + 0x50) = 0x46000000;
  *(undefined4 *)((int)this + 0x148) = 1;
  *(undefined4 *)((int)this + 0x114) = 0;
  *(undefined4 *)((int)this + 0x128) = 0;
  *(undefined4 *)((int)this + 300) = 0;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 0x134) = 0;
  *(undefined4 *)((int)this + 0x138) = 0;
  *(undefined4 *)((int)this + 0x13c) = 0;
  *(undefined4 *)((int)this + 0x140) = 0;
  iVar1 = FUN_00401060();
  iVar1 = FUN_004f3dd0(iVar1,iVar1);
  iVar2 = FUN_005fefd0(10);
  *(int *)((int)this + 0x14c) = iVar2;
  if (iVar2 == 0) {
    pBVar3 = FUN_00401440();
    iVar2 = FUN_005ff020((char *)pBVar3,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x14c) = iVar2;
    if (pBVar3 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar3);
    }
  }
  if (iVar1 != 0) {
    FUN_0052d720(local_1c);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00537ff0(*(void **)((int)this + 0x144),local_1c);
    FUN_006037f0(7);
    for (this_00 = (void *)thunk_FUN_005f5060((int)local_1c); this_00 != (void *)0x0;
        this_00 = *(void **)((int)this_00 + 0x10)) {
      this_03 = (uint *)0x0;
      FUN_00439290(this_00,local_28);
      local_4._0_1_ = 4;
      FUN_0042ec10(this_00,&param_12);
      local_4._0_1_ = 5;
      pvVar4 = (void *)FUN_00618b70(4);
      local_4._0_1_ = 6;
      if (pvVar4 == (void *)0x0) {
        param_9 = 0;
      }
      else {
        param_9 = FUN_004f26d0(pvVar4,&param_12);
      }
      local_4._0_1_ = 5;
      pvVar4 = (void *)FUN_00618b70(0x68);
      local_4._0_1_ = 7;
      if (pvVar4 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        param_10 = param_10 + 1;
        pvVar4 = FUN_0060ba00(pvVar4,param_10);
      }
      local_4._0_1_ = 5;
      if (pvVar4 != (void *)0x0) {
        iVar1 = FUN_0051cab0(&param_12);
        uVar6 = *(uint *)(iVar1 + 0x30);
        pvVar5 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 8;
        if (pvVar5 == (void *)0x0) {
          this_01 = (uint *)0x0;
        }
        else {
          this_01 = (uint *)FUN_005fbd20(pvVar5,*(undefined4 *)((int)this + 0x14c),uVar6 & 0xfff,2);
        }
        local_4._0_1_ = 5;
        pvVar5 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 9;
        if (pvVar5 == (void *)0x0) {
          this_02 = (uint *)0x0;
        }
        else {
          this_02 = (uint *)FUN_005fbda0(pvVar5,0xc3,0x3f,this_01);
        }
        local_4._0_1_ = 5;
        pvVar5 = (void *)FUN_00618b70(0x28);
        local_4._0_1_ = 10;
        if (pvVar5 != (void *)0x0) {
          this_03 = (uint *)FUN_005fbda0(pvVar5,0xc3,0x3f,this_01);
        }
        uVar8 = (undefined1)this_01[2];
        local_4._0_1_ = 5;
        iVar1 = FUN_005fc0f0((int *)this_02);
        uVar6 = FUN_005fc0e0((int *)this_02);
        FUN_005fcfe0(this_02,0,0,uVar6,iVar1,uVar8);
        uVar8 = (undefined1)this_01[2];
        iVar1 = FUN_005fc0f0((int *)this_03);
        uVar6 = FUN_005fc0e0((int *)this_03);
        FUN_005fcfe0(this_03,0,0,uVar6,iVar1,uVar8);
        iVar1 = FUN_005fc0e0((int *)this_01);
        iVar1 = (0xc3 - iVar1) / 2;
        iVar2 = FUN_005fc0f0((int *)this_01);
        iVar2 = (0x3f - iVar2) / 2;
        FUN_005fd0f0(this_01,this_02,iVar1,iVar2);
        FUN_005fd0f0(this_01,this_03,iVar1,iVar2);
        FUN_0060be60(pvVar4,(int)this_02,(int)this_03);
        *(int *)((int)pvVar4 + 0x54) = param_9;
        pcVar7 = (char *)FUN_00583c40((int)local_28);
        FUN_005f35e0((void *)((int)pvVar4 + 0x14),pcVar7);
        iVar1 = FUN_005fc0f0((int *)this_01);
        *(int *)((int)pvVar4 + 0x34) = iVar1;
        *(undefined4 *)((int)pvVar4 + 0x58) = 1;
        *(undefined4 *)((int)pvVar4 + 0x60) = 0x808080;
        *(undefined4 *)((int)pvVar4 + 0x38) = 10;
        if (this_01 != (uint *)0x0) {
          FUN_005fbfa0(this_01);
          FUN_00618b60((undefined *)this_01);
        }
        FUN_005f59f0((void *)((int)this + 0x118),(int)pvVar4);
      }
      local_4._0_1_ = 4;
      FUN_00619730();
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_005f2ff0(local_28);
    }
    local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_0052d760(local_1c);
  }
  ExceptionList = local_c;
  return this;
}


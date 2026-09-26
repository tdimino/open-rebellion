
bool __thiscall FUN_0040bcd0(void *this,int *param_1)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  void *pvVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  uint *puVar10;
  int iVar11;
  bool bVar12;
  bool bVar13;
  float fVar14;
  uint uStack_34;
  uint uStack_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a0c0;
  pvStack_c = ExceptionList;
  iVar11 = 0;
  bVar12 = false;
  if (*(int *)((int)this + 4) != 0) {
    bVar12 = *(int *)((int)this + 8) != 0;
  }
  bVar13 = false;
  ExceptionList = &pvStack_c;
  if (bVar12) {
    ExceptionList = &pvStack_c;
    FUN_0051c270(local_2c,*(undefined4 *)((int)this + 8),1);
    local_4 = 0;
    iVar11 = FUN_00513180((int)local_2c);
    local_4 = 0xffffffff;
    FUN_0051c2f0(local_2c);
    pvVar4 = FUN_005039d0(local_2c,param_1,1);
    local_4 = 1;
    iVar5 = FUN_00513180((int)pvVar4);
    local_4 = 0xffffffff;
    FUN_00503ad0(local_2c);
    iVar6 = (**(code **)(*param_1 + 0x26c))();
    iVar7 = (**(code **)(*param_1 + 0x23c))();
    iVar8 = (**(code **)(*param_1 + 0x274))();
    iVar9 = (**(code **)(*param_1 + 0x248))();
    uVar1 = param_1[0x14];
    puVar10 = FUN_004025b0(param_1,&uStack_30);
    uVar2 = *puVar10;
    local_4 = 2;
    puVar10 = FUN_004025b0(param_1,&uStack_34);
    uVar3 = *puVar10;
    fVar14 = (float)(iVar9 - iVar8);
    iVar5 = (iVar6 - iVar7) + iVar5;
    local_4._0_1_ = 3;
    iVar6 = (**(code **)(*param_1 + 0x248))();
    iVar11 = FUN_00595ef0(uVar3 >> 0x18,uVar2 & 0xffffff,(uint)((uVar1 >> 2 & 1) == 0),
                          (uint)(iVar11 != 0),(float)iVar6,fVar14,iVar5);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar13 = iVar11 != 0;
  }
  if (bVar13) {
    FUN_004f7f60(param_1,iVar11);
  }
  ExceptionList = pvStack_c;
  return bVar13;
}


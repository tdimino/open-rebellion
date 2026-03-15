
bool __thiscall FUN_00606e10(void *this,uint param_1,uint param_2,undefined4 param_3,UINT param_4)

{
  int iVar1;
  void *this_00;
  undefined4 uVar2;
  int *piVar3;
  int iVar4;
  uint uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  int iVar9;
  undefined1 uVar10;
  int iVar11;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006565db;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    piVar3 = (int *)0x0;
  }
  else {
    uVar10 = 2;
    uVar5 = param_1;
    uVar2 = FUN_006037f0(*(int *)((int)this + 0x98));
    piVar3 = (int *)FUN_005fbd20(this_00,uVar2,uVar5,uVar10);
  }
  local_4 = 0xffffffff;
  if (piVar3 != (int *)0x0) {
    iVar11 = 1;
    iVar9 = 0;
    uVar8 = 0;
    iVar1 = *(int *)((int)this + 0x30);
    uVar7 = 0;
    uVar6 = 0;
    uVar5 = 0;
    uVar2 = 3;
    iVar4 = FUN_005fc0e0(piVar3);
    FUN_00606c90(this,(iVar1 - iVar4) + -3,uVar2,param_1,param_2,param_3,param_4,uVar5,uVar6,uVar7,
                 uVar8,iVar9,iVar11);
    FUN_005fbfa0(piVar3);
    FUN_00618b60((undefined *)piVar3);
  }
  ExceptionList = local_c;
  return piVar3 != (int *)0x0;
}


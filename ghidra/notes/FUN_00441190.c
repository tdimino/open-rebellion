
int * __thiscall FUN_00441190(void *this,undefined4 param_1,int param_2)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  int iVar4;
  int iVar5;
  bool bVar6;
  uint *puVar7;
  void *pvVar8;
  int unaff_EBX;
  int *this_00;
  undefined4 uVar9;
  undefined4 uVar10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e9b2;
  local_c = ExceptionList;
  this_00 = (int *)0x0;
  bVar6 = false;
  ExceptionList = &local_c;
  if (param_2 == 0) goto switchD_004411d3_caseD_2;
  ExceptionList = &local_c;
  switch(*(undefined4 *)(param_2 + 4)) {
  case 1:
    ExceptionList = &local_c;
    pvVar8 = (void *)FUN_00618b70(0x1a4);
    local_4 = 0;
    if (pvVar8 == (void *)0x0) {
      this_00 = (int *)0x0;
      bVar6 = true;
    }
    else {
      iVar1 = *(int *)(param_2 + 8);
      uVar2 = *(uint *)(param_2 + 0xc);
      uVar3 = *(undefined4 *)((int)this + 0x1c);
      iVar4 = *(int *)(param_2 + 0x14);
      iVar5 = *(int *)(param_2 + 0x10);
      uVar10 = 0;
      uVar9 = 0;
      puVar7 = (uint *)FUN_005ff440((int)this);
      this_00 = FUN_004591d0(pvVar8,uVar3,iVar1,uVar2,iVar5,iVar4,(int)this,puVar7,uVar9,uVar10);
      bVar6 = true;
    }
    break;
  case 4:
    ExceptionList = &local_c;
    pvVar8 = (void *)FUN_00618b70(0x1b4);
    local_4 = 3;
    if (pvVar8 != (void *)0x0) {
      this_00 = FUN_004a2630(pvVar8,*(undefined4 *)((int)this + 0x1c),*(int *)(param_2 + 8),
                             *(uint *)(param_2 + 0xc),(int)this,0,(void *)0x0);
      bVar6 = true;
      break;
    }
    goto LAB_004413a5;
  case 7:
    ExceptionList = &local_c;
    pvVar8 = (void *)FUN_00618b70(0x118);
    local_4 = 5;
    if (pvVar8 == (void *)0x0) {
      this_00 = (int *)0x0;
    }
    else {
      this_00 = FUN_0049ee20(pvVar8,*(undefined4 *)((int)this + 0x1c),(int)this,0);
    }
    break;
  case 9:
    ExceptionList = &local_c;
    pvVar8 = (void *)FUN_00618b70(0x26c);
    bVar6 = true;
    local_4 = 1;
    if (pvVar8 == (void *)0x0) {
      this_00 = (int *)0x0;
    }
    else {
      this_00 = FUN_00452fc0(pvVar8,*(undefined4 *)((int)this + 0x1c),*(int *)(param_2 + 8),
                             *(uint *)(param_2 + 0xc),*(int *)(param_2 + 0x10),
                             *(uint *)(param_2 + 0x14),(int)this,0,(void *)0x0);
    }
    break;
  case 10:
    ExceptionList = &local_c;
    pvVar8 = (void *)FUN_00618b70(0x184);
    local_4 = 2;
    if (pvVar8 != (void *)0x0) {
      this_00 = FUN_004a7790(pvVar8,*(undefined4 *)((int)this + 0x1c),*(int *)(param_2 + 8),
                             *(uint *)(param_2 + 0xc),*(int *)(param_2 + 0x10),
                             *(int *)(param_2 + 0x14),(int)this,0,(void *)0x0);
      bVar6 = true;
      break;
    }
    goto LAB_004413a5;
  case 0xb:
    ExceptionList = &local_c;
    pvVar8 = (void *)FUN_00618b70(0x1cc);
    local_4 = 4;
    if (pvVar8 != (void *)0x0) {
      this_00 = FUN_0049f130(pvVar8,*(undefined4 *)((int)this + 0x1c),*(int *)(param_2 + 8),
                             *(uint *)(param_2 + 0xc),(uint)this,0,(void *)0x0);
      bVar6 = true;
      break;
    }
LAB_004413a5:
    this_00 = (int *)0x0;
    bVar6 = true;
  }
switchD_004411d3_caseD_2:
  local_4 = 0xffffffff;
  if (this_00 != (int *)0x0) {
    (**(code **)(*this_00 + 0x28))(param_1);
    this_00[0x14] = this_00[0x14] & 0xefffffff;
    FUN_005ffce0(this_00,0);
    FUN_005f4f10((void *)((int)this + 0x6c),(int)this_00);
    if (bVar6) {
      FUN_0042ac70(this,(int)this_00);
    }
    if ((*(int *)(param_2 + 0x18) == 0) && (unaff_EBX != 0)) {
      (**(code **)(*this_00 + 8))();
      FUN_00428b40(this,this_00);
    }
    else {
      (**(code **)(*this_00 + 0x2c))(5);
    }
  }
  ExceptionList = local_c;
  return this_00;
}


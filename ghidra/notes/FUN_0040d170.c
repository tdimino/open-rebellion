
uint __thiscall FUN_0040d170(void *this,int *param_1,void *param_2)

{
  int *this_00;
  void *pvVar1;
  uint uVar2;
  int *piVar3;
  void *pvVar4;
  undefined3 extraout_var;
  int iVar5;
  int *piVar6;
  bool bVar7;
  longlong lVar8;
  int local_14 [5];
  
  this_00 = param_1;
  uVar2 = FUN_004f6df0(param_1,&param_1);
  bVar7 = uVar2 == 0;
  if (!bVar7) {
    piVar3 = (int *)FUN_00596980(local_14);
    piVar6 = local_14;
    for (iVar5 = 5; iVar5 != 0; iVar5 = iVar5 + -1) {
      *piVar6 = *piVar3;
      piVar3 = piVar3 + 1;
      piVar6 = piVar6 + 1;
    }
    bVar7 = uVar2 == 0;
    if (!bVar7) {
      bVar7 = *(int *)((int)this + 4) == 0;
      uVar2 = 0;
      if (!bVar7) {
        uVar2 = (uint)(*(int *)((int)this + 8) != 0);
        bVar7 = uVar2 == 0;
      }
    }
  }
  pvVar1 = param_2;
  if (!bVar7) {
    pvVar4 = (void *)(**(code **)(*this_00 + 0x274))();
    iVar5 = (**(code **)(*this_00 + 0x248))();
    param_2 = pvVar4;
    lVar8 = __ftol();
    iVar5 = (int)lVar8 - (iVar5 - (int)pvVar4);
    if (iVar5 < 1) {
      if (iVar5 < 0) {
        uVar2 = FUN_00501ae0(this_00,-iVar5,0xc,pvVar1);
      }
    }
    else {
      bVar7 = FUN_00501b70(this_00,iVar5,pvVar1);
      uVar2 = CONCAT31(extraout_var,bVar7);
    }
  }
  if (((uVar2 != 0) && (local_14[2] == 0)) && ((*(byte *)(this_00 + 0x14) & 0x40) != 0)) {
    uVar2 = (**(code **)(*this_00 + 0xac))(0xc,pvVar1);
  }
  return uVar2;
}


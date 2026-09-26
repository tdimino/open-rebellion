
int __thiscall FUN_00535e40(void *this,undefined4 param_1,void *param_2)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  void *pvVar7;
  
  iVar3 = FUN_004f2b30(this,param_1,param_2);
  iVar6 = *(int *)((int)this + 0x2c);
  if ((iVar6 == 0) || (iVar3 == 0)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (iVar6 != 0) {
    iVar5 = *(int *)(iVar6 + 0x58);
    pvVar7 = param_2;
    iVar4 = FUN_0053e290(*(int *)(iVar6 + 0x5c));
    iVar5 = FUN_00533e20(this,iVar4 + iVar5,pvVar7);
    if ((iVar5 == 0) || (bVar1 = true, iVar3 == 0)) {
      bVar1 = false;
    }
    iVar3 = *(int *)(iVar6 + 0x60);
    pvVar7 = param_2;
    iVar5 = FUN_0053e290(*(int *)(iVar6 + 100));
    iVar3 = FUN_00533ea0(this,iVar5 + iVar3,pvVar7);
    if ((iVar3 == 0) || (bVar2 = true, !bVar1)) {
      bVar2 = false;
    }
    iVar3 = *(int *)(iVar6 + 0x68);
    pvVar7 = param_2;
    iVar5 = FUN_0053e290(*(int *)(iVar6 + 0x6c));
    iVar3 = FUN_00533f20(this,iVar5 + iVar3,pvVar7);
    if ((iVar3 == 0) || (bVar1 = true, !bVar2)) {
      bVar1 = false;
    }
    iVar3 = *(int *)(iVar6 + 0x70);
    pvVar7 = param_2;
    iVar5 = FUN_0053e290(*(int *)(iVar6 + 0x74));
    iVar3 = FUN_00533fa0(this,iVar5 + iVar3,pvVar7);
    if ((iVar3 == 0) || (bVar2 = true, !bVar1)) {
      bVar2 = false;
    }
    iVar3 = *(int *)(iVar6 + 0x78);
    pvVar7 = param_2;
    iVar5 = FUN_0053e290(*(int *)(iVar6 + 0x7c));
    iVar3 = FUN_00534020(this,iVar5 + iVar3,pvVar7);
    if ((iVar3 == 0) || (bVar1 = true, !bVar2)) {
      bVar1 = false;
    }
    iVar3 = *(int *)(iVar6 + 0x80);
    pvVar7 = param_2;
    iVar5 = FUN_0053e290(*(int *)(iVar6 + 0x84));
    iVar3 = FUN_005340a0(this,iVar5 + iVar3,pvVar7);
    if ((iVar3 == 0) || (bVar2 = true, !bVar1)) {
      bVar2 = false;
    }
    iVar3 = *(int *)(iVar6 + 0x88);
    pvVar7 = param_2;
    iVar5 = FUN_0053e290(*(int *)(iVar6 + 0x8c));
    iVar3 = FUN_00534120(this,iVar5 + iVar3,pvVar7);
    if ((iVar3 == 0) || (bVar1 = true, !bVar2)) {
      bVar1 = false;
    }
    iVar3 = *(int *)(iVar6 + 0x90);
    iVar6 = FUN_0053e290(*(int *)(iVar6 + 0x94));
    iVar6 = FUN_005341a0(this,iVar6 + iVar3,param_2);
    if ((iVar6 != 0) && (bVar1)) {
      return 1;
    }
    iVar3 = 0;
  }
  return iVar3;
}


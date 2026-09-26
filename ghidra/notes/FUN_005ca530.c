
void __thiscall FUN_005ca530(void *this,int param_1)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  void *this_00;
  int *piVar5;
  uint uVar6;
  
  iVar1 = *(int *)((int)this + 0x68);
  if (((((iVar1 == 6) || (*(int *)((int)this + 100) == 6)) || (iVar1 == 2)) ||
      (*(int *)((int)this + 100) == 2)) &&
     (((iVar4 = *(int *)((int)this + 0x50), iVar3 = FUN_005aaf90(),
       *(int *)(*(int *)(iVar3 + 8 + iVar4 * 4) + 8) == 1 &&
       (iVar4 = *(int *)((int)this + 0x50), iVar3 = FUN_005aaf90(), iVar4 != *(int *)(iVar3 + 0x9ec)
       )) && (param_1 != 0)))) {
    return;
  }
  if (*(int *)((int)this + 0x68) == 1) {
    (**(code **)(*(int *)((int)this + 0x98) + 4))();
    FUN_005c9be0(this,1);
  }
  *(int *)((int)this + 0x68) = param_1;
  switch(param_1) {
  case 1:
  case 9:
  case 0xb:
    (**(code **)(*(int *)((int)this + 0x80) + 4))();
    goto switchD_005ca5ad_default;
  case 2:
    iVar4 = (**(code **)(*(int *)this + 4))();
    if (iVar4 != 4) {
LAB_005ca611:
      *(int *)((int)this + 0x68) = iVar1;
      return;
    }
  case 3:
    (**(code **)(*(int *)((int)this + 0x80) + 4))();
    (**(code **)(*(int *)((int)this + 0x98) + 4))();
    goto switchD_005ca5ad_default;
  case 6:
    iVar4 = (**(code **)(*(int *)this + 4))();
    if ((iVar4 != 4) || (*(int *)((int)this + 0x50) != 0)) goto LAB_005ca611;
  case 0:
  case 4:
  case 5:
    (**(code **)(*(int *)((int)this + 0x80) + 4))();
    piVar5 = (int *)((int)this + 0x98);
    break;
  case 7:
  case 8:
    piVar5 = (int *)((int)this + 0x80);
    break;
  case 10:
    piVar5 = (int *)((int)this + 0x74);
    break;
  default:
    goto switchD_005ca5ad_default;
  }
  (**(code **)(*piVar5 + 4))();
switchD_005ca5ad_default:
  if (((param_1 == 2) || (param_1 == 6)) &&
     (iVar1 = *(int *)((int)this + 0x50), iVar4 = FUN_005aaf90(), iVar1 == *(int *)(iVar4 + 0x9ec)))
  {
    uVar6 = *(uint *)((int)this + 0x18);
    this_00 = (void *)FUN_0059ed70();
    FUN_005a0320(this_00,uVar6);
  }
  iVar1 = *(int *)((int)this + 0x50);
  iVar4 = FUN_005aaf90();
  iVar4 = *(int *)(iVar4 + 0x9ec);
  uVar2 = *(undefined4 *)((int)this + 0x18);
  piVar5 = (int *)FUN_005aaf90();
  (**(code **)(*piVar5 + 0x28))(10,uVar2,param_1,iVar1 == iVar4);
  return;
}


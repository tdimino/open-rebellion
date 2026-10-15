
void __thiscall FUN_00450540(int *param_1,undefined2 param_2,uint param_3)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  
  switch(param_2) {
  case 0x65:
    piVar3 = (int *)param_1[0x45];
    iVar2 = 0;
    iVar1 = 1;
    break;
  case 0x66:
    piVar3 = (int *)param_1[0x45];
    iVar2 = 0;
    iVar1 = 0;
    break;
  case 0x67:
    piVar3 = (int *)param_1[0x45];
    iVar2 = 1;
    iVar1 = 1;
    break;
  default:
    goto switchD_0045055c_caseD_68;
  case 200:
    FUN_00450660(param_1,param_3 >> 0x10);
    return;
  }
  FUN_0041ce60(iVar1,iVar2,piVar3);
  FUN_00417640();
  (**(code **)(*param_1 + 0x30))();
  FUN_00451050((int)param_1);
switchD_0045055c_caseD_68:
  return;
}


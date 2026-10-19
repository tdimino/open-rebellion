
bool FUN_0054ee90(int *param_1,int *param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  bVar1 = true;
  if ((param_1 != (int *)0x0) && (param_2 != (int *)0x0)) {
    iVar2 = (**(code **)(*param_1 + 0x28))();
    iVar3 = (**(code **)(*param_2 + 0x28))();
    bVar1 = iVar2 < iVar3;
  }
  return bVar1;
}


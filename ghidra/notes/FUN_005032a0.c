
int __fastcall FUN_005032a0(int *param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = (**(code **)(*param_1 + 600))();
  iVar2 = (**(code **)(*param_1 + 0x248))();
  return iVar2 - iVar1;
}


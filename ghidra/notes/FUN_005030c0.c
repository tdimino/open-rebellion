
int __fastcall FUN_005030c0(int *param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = FUN_00502f20((int)param_1);
  iVar2 = (**(code **)(*param_1 + 0x244))();
  return iVar2 * iVar1;
}



undefined4 __cdecl
FUN_0055eb00(int param_1,int param_2,int param_3,int param_4,uint *param_5,uint *param_6)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  *param_5 = 0;
  *param_6 = 0;
  iVar1 = param_2 / DAT_006bb780;
  iVar2 = param_4 / DAT_006bb780;
  iVar3 = FUN_0053e290(DAT_006bb78c);
  iVar3 = iVar3 + ((iVar1 + param_1) - (iVar2 + param_3));
  *param_5 = (uint)(iVar3 <= DAT_006bb778);
  *param_6 = (uint)(DAT_006bb784 <= iVar3);
  return 1;
}


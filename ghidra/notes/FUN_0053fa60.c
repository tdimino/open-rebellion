
uint __cdecl FUN_0053fa60(uint param_1,uint param_2,void *param_3,int *param_4,void *param_5)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = *param_4;
  uVar2 = 1;
  FUN_00540200(param_4,param_2);
  if ((*param_4 != iVar1) && ((*(byte *)(param_4 + 1) & 1) != 0)) {
    uVar2 = FUN_0053fb00(param_1,param_3,param_4,param_5);
  }
  return uVar2;
}


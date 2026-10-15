
int __cdecl FUN_00518a70(uint *param_1,void *param_2)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = 1;
  for (pvVar1 = (void *)FUN_005069d0(); pvVar1 != (void *)0x0; pvVar1 = (void *)FUN_00518ff0(pvVar1)
      ) {
    iVar2 = FUN_00518b20(pvVar1,param_1,param_2);
    if ((iVar2 == 0) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  return iVar3;
}



uint __thiscall FUN_0049df20(void *this,uint param_1,undefined4 param_2)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  
  uVar3 = 0;
  iVar1 = FUN_005f5500(this,param_1);
  if (iVar1 == 0) {
    piVar2 = FUN_0049e010(this,param_1);
    if (piVar2 != (int *)0x0) {
      (**(code **)(*piVar2 + 0xc))(param_2);
      piVar2[7] = 0;
      uVar3 = FUN_005f5440(this,piVar2);
    }
  }
  return uVar3;
}


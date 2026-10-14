
void __cdecl
FUN_005c12d0(uint param_1,uint param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5)

{
  int *piVar1;
  int iVar2;
  
  if (DAT_006bc700 != (void *)0x0) {
    piVar1 = (int *)FUN_005c1cd0(DAT_006bc700,param_1);
    iVar2 = FUN_005c1cd0(DAT_006bc700,param_2);
    if ((piVar1 != (int *)0x0) && (iVar2 != 0)) {
      (**(code **)(*piVar1 + 0x20))(iVar2,param_3,param_4,param_5);
    }
  }
  return;
}


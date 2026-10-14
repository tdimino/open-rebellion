
void __thiscall
FUN_00442070(int param_1,uint param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5,
            undefined4 param_6)

{
  int *piVar1;
  
  piVar1 = (int *)FUN_005f5500((void *)(param_1 + 0x18),param_2);
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(param_4,param_3,param_1,param_5,param_6,0);
  }
  return;
}


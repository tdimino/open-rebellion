
undefined4 __thiscall
FUN_00472930(int *param_1,int param_2,int param_3,undefined4 param_4,undefined4 param_5,
            undefined4 param_6,int param_7)

{
  int *piVar1;
  undefined1 auStack_10 [16];
  
  piVar1 = (int *)FUN_00403ed0(param_2);
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 0xc))(param_1);
    piVar1[3] = param_3;
    (**(code **)(*piVar1 + 0x10))(param_2);
    (**(code **)(*piVar1 + 0x14))(param_3,param_4);
  }
  FUN_005f5e40(param_1 + 0xc,(int)piVar1);
  if ((int *)param_1[7] != (int *)0x0) {
    (**(code **)(*(int *)param_1[7] + 0x24))();
    return 1;
  }
  if (param_7 == 0) {
    (**(code **)(*param_1 + 0xc))(auStack_10,&param_2);
  }
  return 1;
}



int __thiscall FUN_00594f30(void *this,int *param_1)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  byte *pbVar4;
  int local_74;
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650c08;
  pvStack_c = ExceptionList;
  local_74 = 0;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  iVar1 = FUN_0051ebb0();
  iVar3 = 0;
  if (iVar1 != 0) {
    FUN_00584e60(param_1,local_70,&local_74);
    pbVar4 = (byte *)((int)this + 8);
    piVar2 = FUN_00584e60(param_1,local_70,&local_74);
    FUN_00617b90(piVar2,(int)pbVar4);
    FUN_00584fd0(param_1,&local_74);
    iVar3 = FUN_0058f900((byte *)((int)this + 8));
    if ((iVar3 != 0) &&
       (FUN_00585070(param_1), *(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0)) {
      do {
        piVar2 = (int *)FUN_0058f800((byte *)((int)this + 8));
        if (piVar2 != (int *)0x0) {
          iVar1 = (**(code **)(*piVar2 + 0xc))(param_1);
          if (iVar1 == 0) {
            (**(code **)*piVar2)(1);
          }
          else {
            FUN_00595070(this,piVar2);
          }
        }
      } while (*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0);
    }
  }
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return iVar3;
}


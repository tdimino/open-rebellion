
int __thiscall FUN_0058e870(void *this,int *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  undefined2 *puVar5;
  int local_90;
  undefined2 local_8c;
  undefined2 local_8a;
  int local_88;
  int local_84;
  int local_80;
  int local_7c;
  int local_78;
  undefined1 *local_74;
  int local_70 [5];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006501a0;
  pvStack_c = ExceptionList;
  local_90 = 0;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  local_4 = 0;
  FUN_00520570(&local_80);
  local_4 = CONCAT31(local_4._1_3_,1);
  local_8c = 0;
  local_8a = 0;
  iVar1 = FUN_0051ebb0();
  local_78 = iVar1;
  if (iVar1 != 0) {
    FUN_00584e60(param_1,local_70,&local_90);
    piVar3 = &local_88;
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar2,piVar3);
    piVar3 = &local_84;
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar2,piVar3);
    *(int *)((int)this + 8) = local_88;
    *(int *)((int)this + 0xc) = local_84;
    FUN_00584fd0(param_1,&local_90);
    FUN_00584e60(param_1,local_70,&local_90);
    piVar3 = &local_88;
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar2,piVar3);
    piVar3 = &local_84;
    piVar2 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_00617760(piVar2,piVar3);
    local_80 = local_88;
    local_7c = local_84;
    FUN_00584fd0(param_1,&local_90);
    FUN_00584e60(param_1,local_70,&local_90);
    puVar5 = &local_8a;
    piVar3 = FUN_00584e60(param_1,local_70,&local_90);
    FUN_006176a0(piVar3,puVar5);
    FUN_00584fd0(param_1,&local_90);
    if (iVar1 != 0) {
      iVar4 = 0x58ea07;
      piVar3 = param_1;
      FUN_00585070(param_1);
      if (*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0) {
        do {
          local_74 = &stack0xffffff58;
          FUN_00520580(&stack0xffffff58,(undefined4 *)((int)this + 8));
          piVar2 = (int *)FUN_00584ac0(iVar4,(int)piVar3);
          if (piVar2 != (int *)0x0) {
            FUN_00520580(piVar2 + 9,&local_80);
            *(undefined2 *)((int)piVar2 + 0x32) = local_8a;
            iVar1 = (**(code **)(*piVar2 + 0xc))();
            if (iVar1 == 0) {
              piVar3 = (int *)0x1;
              iVar4 = 0x58ea69;
              (**(code **)*piVar2)();
            }
            else {
              iVar4 = 0x58ea5f;
              FUN_0058ead0(this,piVar2);
              piVar3 = piVar2;
            }
          }
          iVar1 = local_78;
        } while (*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0);
      }
    }
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00615ea0((int)auStack_5c);
  FUN_00615440(auStack_5c);
  ExceptionList = pvStack_c;
  return iVar1;
}


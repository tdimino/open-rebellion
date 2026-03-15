
void __fastcall FUN_005c19e0(undefined4 *param_1)

{
  undefined *puVar1;
  int *piVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00653a34;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066c960;
  puVar1 = (undefined *)param_1[0x18b];
  local_4 = 10;
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar1 = (undefined *)param_1[0x18c];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar1 = (undefined *)param_1[0x18d];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar6 = param_1 + 0x176;
  piVar2 = (int *)thunk_FUN_005f5060((int)puVar6);
  while (piVar2 != (int *)0x0) {
    FUN_005f54d0(puVar6,piVar2[6]);
    (**(code **)(*piVar2 + 4))();
    if (piVar2 != (int *)0x0) {
      (**(code **)*piVar2)(1);
    }
    piVar2 = (int *)thunk_FUN_005f5060((int)puVar6);
  }
  if (param_1[0x175] == 0) {
    puVar6 = (undefined4 *)0x0;
  }
  else {
    puVar6 = (undefined4 *)(param_1[0x175] + -0x1c);
  }
  if (puVar6 != (undefined4 *)0x0) {
    do {
      if (puVar6[9] == 0) {
        puVar5 = (undefined4 *)0x0;
      }
      else {
        puVar5 = (undefined4 *)(puVar6[9] + -0x1c);
      }
      if (puVar6 == (undefined4 *)0x0) {
        puVar3 = (undefined4 *)0x0;
      }
      else {
        puVar3 = puVar6 + 7;
      }
      FUN_005f5ac0(param_1 + 0x173,(int)puVar3);
      if (puVar6 != (undefined4 *)0x0) {
        (**(code **)*puVar6)(1);
      }
      puVar6 = puVar5;
    } while (puVar5 != (undefined4 *)0x0);
  }
  FUN_005f5b20(&DAT_006bcbc0);
  if ((undefined4 *)param_1[399] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[399])(1);
  }
  if ((undefined4 *)param_1[400] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[400])(1);
  }
  if ((undefined4 *)param_1[0x191] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x191])(1);
  }
  iVar4 = (**(code **)(*(int *)param_1[0x4e] + 8))((int *)param_1[0x4e]);
  while (iVar4 != 0) {
    iVar4 = (**(code **)(*(int *)param_1[0x4e] + 8))((int *)param_1[0x4e]);
  }
  puStack_8._1_3_ = (uint3)((uint)puStack_8 >> 8);
  puStack_8._0_1_ = 9;
  FUN_005fbba0(param_1 + 0x198);
  puStack_8._0_1_ = 8;
  FUN_005f5870(param_1 + 0x185);
  puStack_8._0_1_ = 7;
  FUN_005f5870(param_1 + 0x181);
  puStack_8._0_1_ = 6;
  FUN_005f5870(param_1 + 0x17d);
  puStack_8._0_1_ = 5;
  FUN_005f5870(param_1 + 0x179);
  puStack_8._0_1_ = 4;
  FUN_005f5300(param_1 + 0x176);
  puStack_8._0_1_ = 3;
  FUN_005f2f40(param_1 + 0x173);
  puStack_8._0_1_ = 2;
  FUN_005d5c20(param_1 + 0x66);
  puStack_8._0_1_ = 1;
  FUN_005d6350(param_1 + 0x60);
  puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
  FUN_005d5fc0(param_1 + 0x5a);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_005d65e0(param_1);
  ExceptionList = param_1;
  return;
}


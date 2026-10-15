
void __thiscall FUN_004960f0(int param_1,int param_2,uint *param_3,int param_4)

{
  uint *puVar1;
  int *piVar2;
  uint uVar3;
  int *piVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  bool bVar7;
  undefined4 auStack_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  puVar1 = param_3;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00636208;
  pvStack_c = ExceptionList;
  bVar7 = false;
  if (param_2 == 0x14e) {
    ExceptionList = &pvStack_c;
    piVar2 = FUN_004f3220(*(int *)(param_1 + 0x20),param_3);
    param_2 = 0;
    param_3 = (uint *)0x0;
    FUN_004f26d0((void *)(param_1 + 0x5c),puVar1);
    if ((piVar2 == (int *)0x0) || (param_4 == 0)) {
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
    else {
      FUN_004ffdd0(auStack_2c,piVar2);
      uStack_4 = 0;
      FUN_00513120((int)auStack_2c);
      param_3 = (uint *)FUN_0052bed0((int)auStack_2c);
      if (param_3 == (uint *)0x0) {
        *(undefined4 *)(param_1 + 0x40) = 0;
      }
      uVar3 = (uint)piVar2[9] >> 6 & 3;
      if (uVar3 == 1) {
        param_2 = FUN_004f3dd0(*(int *)(param_1 + 0x20),2);
        bVar7 = *(int *)(param_1 + 0x20) == 1;
        *(undefined2 *)(param_1 + 0x2e) = 0x404;
      }
      else if (uVar3 == 2) {
        param_2 = FUN_004f3dd0(*(int *)(param_1 + 0x20),1);
        bVar7 = *(int *)(param_1 + 0x20) == 2;
        *(undefined2 *)(param_1 + 0x2e) = 0x403;
      }
      else {
        bVar7 = false;
        param_2 = 0;
        *(undefined4 *)(param_1 + 0x40) = 0;
      }
      if ((piVar2[0x22] & 0x4000U) == 0) {
        *(undefined4 *)(param_1 + 0x40) = 1;
      }
      else {
        *(undefined4 *)(param_1 + 0x40) = 0;
      }
      uStack_4 = 0xffffffff;
      FUN_004fff70(auStack_2c);
    }
    if (param_2 == 0) {
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
    if (*(int *)(param_1 + 0x40) != 0) {
      *(undefined2 *)(param_1 + 0x30) = 0x45e;
      if (bVar7) {
        if (piVar2 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar2 + 0xc;
        }
        FUN_0060b9d0((void *)(param_1 + 0x44),0x70a2,piVar4,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        if (param_3 == (uint *)0x0) {
          puVar6 = (undefined4 *)0x0;
        }
        else {
          puVar6 = (undefined4 *)((int)param_3 + 0x30);
        }
        if (piVar2 == (int *)0x0) {
          piVar2 = (int *)0x0;
        }
        else {
          piVar2 = piVar2 + 0xc;
        }
        if (param_2 == 0) {
          puVar5 = (undefined4 *)0x0;
        }
        else {
          puVar5 = (undefined4 *)(param_2 + 0x30);
        }
        FUN_0060b9d0((void *)(param_1 + 0x50),0x70a3,puVar5,piVar2,puVar6,(undefined4 *)0x0);
        *(undefined4 *)(param_1 + 0x28) = 0xe;
      }
      else {
        if (piVar2 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar2 + 0xc;
        }
        FUN_0060b9d0((void *)(param_1 + 0x44),0x70a0,piVar4,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        if (param_3 == (uint *)0x0) {
          puVar6 = (undefined4 *)0x0;
        }
        else {
          puVar6 = (undefined4 *)((int)param_3 + 0x30);
        }
        if (piVar2 == (int *)0x0) {
          piVar2 = (int *)0x0;
        }
        else {
          piVar2 = piVar2 + 0xc;
        }
        if (param_2 == 0) {
          puVar5 = (undefined4 *)0x0;
        }
        else {
          puVar5 = (undefined4 *)(param_2 + 0x30);
        }
        FUN_0060b9d0((void *)(param_1 + 0x50),0x70a1,puVar5,piVar2,puVar6,(undefined4 *)0x0);
        *(undefined4 *)(param_1 + 0x28) = 0xd;
      }
    }
  }
  ExceptionList = pvStack_c;
  return;
}


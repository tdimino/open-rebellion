
void __thiscall FUN_00438800(int *param_1,uint param_2)

{
  undefined4 *puVar1;
  uint uVar2;
  int iVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062d708;
  pvStack_c = ExceptionList;
  uVar2 = param_2 & 0xffff;
  if (uVar2 < 0x29c) {
    if (uVar2 == 0x29b) {
      ExceptionList = &pvStack_c;
      FUN_00439160(param_1);
    }
    else {
      switch(uVar2) {
      case 100:
      case 0x66:
        ExceptionList = &pvStack_c;
        if ((undefined4 *)param_1[0x51] != (undefined4 *)0x0) {
          ExceptionList = &pvStack_c;
          (*(code *)**(undefined4 **)param_1[0x51])(1);
          param_1[0x51] = 0;
        }
        (**(code **)(*param_1 + 0x30))();
        ExceptionList = pvStack_c;
        return;
      case 0x65:
        ExceptionList = &pvStack_c;
        FUN_00438980((int)param_1);
        (**(code **)(*param_1 + 0x30))();
        ExceptionList = pvStack_c;
        return;
      case 0x67:
        ExceptionList = &pvStack_c;
        iVar3 = FUN_0060c960(param_1[0x45]);
        if (iVar3 != 0) {
          puVar1 = *(undefined4 **)(iVar3 + 0x54);
          FUN_004ece30(&param_2);
          uStack_4 = 0;
          FUN_0041d6b0(&param_2,puVar1);
          uStack_4 = 0xffffffff;
          FUN_00619730();
          ExceptionList = pvStack_c;
          return;
        }
        break;
      case 0x69:
        ExceptionList = &pvStack_c;
        FUN_00438c30(param_1);
        ExceptionList = pvStack_c;
        return;
      case 0x6a:
        ExceptionList = &pvStack_c;
        FUN_00438c60(param_1);
        ExceptionList = pvStack_c;
        return;
      case 0x6c:
        if (param_1[0x45] != 0) {
          ExceptionList = &pvStack_c;
          FUN_0060c1f0(param_1[0x45]);
          ExceptionList = pvStack_c;
          return;
        }
      }
    }
  }
  ExceptionList = pvStack_c;
  return;
}


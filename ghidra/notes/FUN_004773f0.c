
int __fastcall FUN_004773f0(void *param_1)

{
  int iVar1;
  void *pvVar2;
  int iVar3;
  int *piVar4;
  undefined4 local_4;
  
  local_4 = 0;
  pvVar2 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x58) + 0x44),(uint *)((int)param_1 + 0x34)
                       );
  if (pvVar2 != (void *)0x0) {
    if (*(int **)((int)param_1 + 0x5c) != (int *)0x0) {
      iVar1 = *(int *)((int)pvVar2 + 0x30);
      iVar3 = (**(code **)(**(int **)((int)param_1 + 0x5c) + 0x14))();
      if (iVar3 == iVar1) {
        local_4 = 1;
      }
      else {
        FUN_00619730();
        if (*(undefined4 **)((int)param_1 + 0x5c) != (undefined4 *)0x0) {
          (**(code **)**(undefined4 **)((int)param_1 + 0x5c))(1);
        }
        *(undefined4 *)((int)param_1 + 0x5c) = 0;
      }
    }
    if (*(int *)((int)param_1 + 0x5c) == 0) {
      FUN_004fcdd0((undefined4 *)((int)param_1 + 0x60));
      FUN_004fcdd0((undefined4 *)((int)param_1 + 100));
      *(undefined4 *)((int)param_1 + 0x1c) = 0;
      *(undefined4 *)((int)param_1 + 0x68) = 0;
      if (*(int *)((int)pvVar2 + 0x30) != 0) {
        piVar4 = (int *)FUN_00477630(param_1,*(int *)((int)pvVar2 + 0x30));
        *(int **)((int)param_1 + 0x5c) = piVar4;
        if (piVar4 != (int *)0x0) {
          iVar1 = *(int *)((int)pvVar2 + 0x30);
          iVar3 = (**(code **)(*piVar4 + 0x14))();
          if (iVar3 == iVar1) {
            local_4 = 1;
          }
        }
      }
    }
    if (*(int *)((int)pvVar2 + 0x3c) != *(int *)((int)param_1 + 0x44)) {
      FUN_004769f0(param_1,*(int *)((int)pvVar2 + 0x3c));
    }
  }
  if (local_4 == 0) {
    FUN_00477190(param_1);
  }
  return local_4;
}


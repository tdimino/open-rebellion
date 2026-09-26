
uint FUN_0053c8d0(void)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  undefined4 unaff_EBX;
  int *unaff_ESI;
  void *unaff_EDI;
  undefined4 in_stack_00000014;
  undefined4 in_stack_00000018;
  void *in_stack_0000001c;
  undefined4 in_stack_00000028;
  undefined4 *in_stack_00000030;
  void *in_stack_00000034;
  
  if (in_stack_00000030 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_004f5440(in_stack_00000030);
  }
  in_stack_00000030 = (undefined4 *)(uint)(puVar1 != (undefined4 *)0x0);
  *unaff_ESI = (int)puVar1;
  if (in_stack_00000030 != (undefined4 *)0x0) {
    FUN_005f5600(puVar1,*(undefined4 *)(*(int *)((int)unaff_EDI + 0x44) + 8));
    *(undefined4 *)(*unaff_ESI + 0x1c) = *(undefined4 *)((int)unaff_EDI + 0x1c);
    *(undefined4 *)(*unaff_ESI + 0x20) = *(undefined4 *)((int)unaff_EDI + 0x20);
    *(undefined4 *)(*unaff_ESI + 0x44) = 0;
    *(undefined4 *)(*unaff_ESI + 0x48) = 0;
    uVar2 = FUN_0053ca30(unaff_EDI,&stack0x0000002c);
    (**(code **)(*(int *)*unaff_ESI + 0x2c))(uVar2);
    FUN_00619730();
    in_stack_00000028 = 0x41000001;
    FUN_004f26d0((void *)(*unaff_ESI + 0x4c),&stack0x00000028);
    FUN_00619730();
    (**(code **)(*(int *)*unaff_ESI + 0x24))(&stack0x00000008);
    puVar1 = FUN_004f4340(&stack0x00000014);
    FUN_004f43b0((void *)(*unaff_ESI + 0x58),puVar1);
    FUN_004f4380(&stack0x00000014);
  }
  if (in_stack_00000030 == (undefined4 *)0x0) {
    in_stack_00000014 = unaff_EBX;
    in_stack_00000018 = unaff_EBX;
    FUN_00520580(in_stack_00000034,&stack0x00000014);
    FUN_00619730();
  }
  if ((*(int *)((int)in_stack_00000034 + 4) != -1) &&
     ((undefined4 *)*unaff_ESI != (undefined4 *)0x0)) {
    (*(code *)**(undefined4 **)*unaff_ESI)();
    *unaff_ESI = 0;
  }
  FUN_004f4380((undefined4 *)&stack0x0000000c);
  ExceptionList = in_stack_0000001c;
  return (uint)in_stack_00000030;
}


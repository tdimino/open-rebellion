
uint __cdecl FUN_005535b0(int *param_1,int param_2,int *param_3,void *param_4)

{
  int *this;
  uint uVar1;
  void *this_00;
  undefined4 *puVar2;
  undefined4 *puVar3;
  void *this_01;
  bool bVar4;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  this = param_3;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00649030;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_3 + 4))();
  this_01 = param_4;
  uStack_1c = 1;
  uStack_18 = 0xffffffff;
  uStack_4 = 0;
  FUN_00520580(param_4,&uStack_1c);
  uStack_4 = 0xffffffff;
  FUN_00619730();
  FUN_004f4340(auStack_14);
  uStack_4 = 1;
  uVar1 = FUN_0053f150(param_1 + 0x10,param_2,auStack_14,this_01);
  if (uVar1 != 0) {
    if (*(int *)((int)this_01 + 4) == -1) {
      this_00 = (void *)thunk_FUN_005f5060((int)auStack_14);
      this_01 = param_4;
      while ((param_4 = this_01, this_00 != (void *)0x0 && (bVar4 = uVar1 != 0, uVar1 = 0, bVar4)))
      {
        puVar2 = (undefined4 *)(**(code **)(*param_1 + 4))();
        uVar1 = 0;
        if (puVar2 != (undefined4 *)0x0) {
          uVar1 = puVar2[6];
          if (uVar1 == 0) {
            if (puVar2 != (undefined4 *)0x0) {
              (**(code **)*puVar2)(1);
            }
          }
          else {
            puVar3 = FUN_00403040(this_00,&param_3);
            uStack_4._0_1_ = 2;
            FUN_004f26d0(puVar2 + 0xf,puVar3);
            uStack_4 = CONCAT31(uStack_4._1_3_,1);
            FUN_00619730();
            FUN_0054ee30(this,(int)puVar2);
          }
        }
        this_00 = *(void **)((int)this_00 + 0x10);
        this_01 = param_4;
      }
    }
    if (uVar1 != 0) goto LAB_005536f9;
  }
  uStack_1c = 1;
  uStack_18 = 1;
  uStack_4._0_1_ = 3;
  FUN_00520580(this_01,&uStack_1c);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_00619730();
LAB_005536f9:
  uStack_4 = 0xffffffff;
  FUN_004f4380(auStack_14);
  ExceptionList = pvStack_c;
  return uVar1;
}


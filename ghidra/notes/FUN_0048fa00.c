
void __thiscall FUN_0048fa00(int param_1,uint param_2,undefined4 param_3,int param_4)

{
  bool bVar1;
  bool bVar2;
  ushort uVar3;
  int *piVar4;
  uint *puVar5;
  void *this;
  undefined4 uStack_1c;
  uint auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  puStack_8 = &LAB_00635a50;
  pvStack_c = ExceptionList;
  if ((param_2 == 0x212) && (param_4 == 0)) {
    param_2 = 0x33000243;
    uStack_4 = 0;
    ExceptionList = &pvStack_c;
    piVar4 = FUN_004f2ec0(*(int *)(param_1 + 0x20),&param_2);
    uStack_4 = 0xffffffff;
    FUN_00619730();
    *(undefined4 *)(param_1 + 0x40) = 1;
    if ((*(byte *)(piVar4 + 0x2b) & 1) == 0) {
      if (*(int *)(param_1 + 0x20) == 1) {
        *(undefined2 *)(param_1 + 0x2e) = 0x41c;
        *(undefined2 *)(param_1 + 0x30) = 0x475;
        *(undefined4 *)(param_1 + 0x28) = 0x19;
        uVar3 = FUN_004c4990(&DAT_00000120);
        *(ushort *)(param_1 + 0x32) = uVar3;
        FUN_0060b9d0((void *)(param_1 + 0x44),0x7168,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
        FUN_0060b9d0((void *)(param_1 + 0x50),0x7169,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
        param_4 = 0x33000243;
        uStack_4 = 1;
        FUN_004f26d0((void *)(param_1 + 0x5c),&param_4);
        uStack_4 = 0xffffffff;
        FUN_00619730();
      }
      else {
        this = (void *)piVar4[7];
        bVar2 = false;
        if (this != (void *)0x0) {
          do {
            if (bVar2) break;
            puVar5 = FUN_004025b0(this,auStack_18);
            auStack_18[1] = 0x90;
            auStack_18[2] = 0x98;
            uStack_4 = 2;
            if ((*puVar5 >> 0x18 < 0x90) || (0x97 < *puVar5 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            uStack_4 = 0xffffffff;
            FUN_00619730();
            if (bVar1) {
              bVar2 = true;
            }
            else {
              this = *(void **)((int)this + 0x1c);
            }
          } while (this != (void *)0x0);
          if (this != (void *)0x0) {
            *(undefined2 *)(param_1 + 0x2e) = 0x41e;
            *(undefined2 *)(param_1 + 0x30) = 0x477;
            *(undefined4 *)(param_1 + 0x28) = 0x2d;
            FUN_0060b9d0((void *)(param_1 + 0x44),0x716a,(undefined4 *)0x0,(undefined4 *)0x0,
                         (undefined4 *)0x0,(undefined4 *)0x0);
            FUN_0060b9d0((void *)(param_1 + 0x50),0x716b,(undefined4 *)((int)this + 0x30),
                         (undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
            uStack_1c = 0x33000243;
            uStack_4 = 3;
            FUN_004f26d0((void *)(param_1 + 0x5c),&uStack_1c);
            uStack_4 = 0xffffffff;
            FUN_00619730();
            ExceptionList = pvStack_c;
            return;
          }
        }
        *(undefined4 *)(param_1 + 0x40) = 0;
      }
    }
    else {
      *(undefined2 *)(param_1 + 0x2e) = 0x41d;
      *(undefined2 *)(param_1 + 0x30) = 0x476;
      *(undefined4 *)(param_1 + 0x28) = 0x21;
      uVar3 = FUN_004c4990(&DAT_0000011f);
      *(ushort *)(param_1 + 0x32) = uVar3;
      FUN_0060b9d0((void *)(param_1 + 0x44),0x716c,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
      FUN_0060b9d0((void *)(param_1 + 0x50),0x716d,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

